extern crate diesel;
extern crate dotenv;

embed_migrations!();

use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::Connection;
use diesel::QueryResult;
use r2d2::{Pool, PooledConnection};
use tokio::task;

use crate::errors::PgRepoError;
use crate::errors::PgRepoErrorType;

#[derive(Clone)]
pub struct Repo {
    pub(crate) pool: Pool<ConnectionManager<PgConnection>>,
}

pub struct RepoConnection {
    pub(crate) pg_conn: PooledConnection<ConnectionManager<PgConnection>>,
}

impl Repo {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }

    pub fn get_conn(&self) -> Result<RepoConnection, PgRepoError> {
        Ok(RepoConnection {
            pg_conn: self.pool.get()?,
        })
    }

    pub async fn transaction<R, Func>(&self, f: Func) -> Result<R, PgRepoError>
    where
        R: Send,
        Func: FnOnce(&RepoConnection) -> QueryResult<R> + Send,
    {
        task::block_in_place(move || {
            let conn = self.get_conn()?;
            let result = conn.pg_conn.transaction(|| f(&conn));
            Ok(result?)
        })
    }
}

impl RepoConnection {
    pub fn new(repo: Repo) -> Result<RepoConnection, PgRepoError> {
        repo.get_conn()
    }

    pub async fn transaction<R, E, Func>(&self, f: Func) -> Result<R, PgRepoError>
    where
        R: Send,
        Func: FnOnce(&RepoConnection) -> QueryResult<R> + Send,
        E: Into<PgRepoError>,
    {
        task::block_in_place(move || {
            let conn = &self;
            let result = conn.pg_conn.build_transaction().run(|| f(conn));
            Ok(result?)
        })
    }
}

pub fn get_pg_pool(database_url: String, max_size: u32) -> Repo {
    let manager = ConnectionManager::new(database_url.as_str());
    let pool = r2d2::Builder::default()
        .max_size(max_size)
        .build(manager)
        .expect("Could not instantiate db pool");
    let conn = pool.get().expect("Could not get db pool connection");
    embedded_migrations::run_with_output(&conn, &mut std::io::stdout())
        .expect("Could not run migrations");
    Repo { pool }
}

pub fn get_roll_back_err() -> diesel::result::Error {
    diesel::result::Error::RollbackTransaction
}
