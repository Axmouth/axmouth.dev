extern crate diesel;
extern crate dotenv;

embed_migrations!();

use std::sync::Arc;

use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::QueryResult;
use r2d2::{Pool, PooledConnection};
use tokio::task;

use crate::errors::PgRepoError;

pub trait Repo {
    fn get_pool_connection(
        &self,
    ) -> Result<PooledConnection<ConnectionManager<PgConnection>>, PgRepoError>;

    fn get_pool(&self) -> Pool<ConnectionManager<PgConnection>>;

    fn get_conn(&self) -> Result<RepoConnection, PgRepoError>;
}

#[derive(Clone)]
pub struct PgRepo {
    pub(crate) pool: Pool<ConnectionManager<PgConnection>>,
}

pub type DynRepo = Arc<dyn Repo + Send + Sync>;

impl Repo for PgRepo {
    fn get_pool_connection(
        &self,
    ) -> Result<PooledConnection<ConnectionManager<PgConnection>>, PgRepoError> {
        let conn = self.pool.get()?;
        Ok(conn)
    }

    fn get_pool(&self) -> Pool<ConnectionManager<PgConnection>> {
        self.pool.clone()
    }

    fn get_conn(&self) -> Result<RepoConnection, PgRepoError> {
        Ok(RepoConnection {
            pg_conn: self.pool.get()?,
        })
    }
}

pub struct RepoConnection {
    pub(crate) pg_conn: PooledConnection<ConnectionManager<PgConnection>>,
}

impl PgRepo {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }

    pub async fn transaction<R, Func>(&self, f: Func) -> Result<R, PgRepoError>
    where
        R: Send + Sized,
        Func: FnOnce(&RepoConnection) -> QueryResult<R> + Send + Sized,
    {
        task::block_in_place(move || {
            let conn = self.get_conn()?;
            let result = conn
                .pg_conn
                .build_transaction()
                .repeatable_read()
                .run(|| f(&conn));
            Ok(result?)
        })
    }
}

impl RepoConnection {
    pub fn new<R>(repo: R) -> Result<RepoConnection, PgRepoError>
    where
        R: Repo + Send + Sync,
    {
        repo.get_conn()
    }
}

pub async fn pg_transaction<ResultType, Func>(
    repo: DynRepo,
    f: Func,
) -> Result<ResultType, PgRepoError>
where
    ResultType: Send + Sized,
    Func: FnOnce(&RepoConnection) -> QueryResult<ResultType> + Send + Sized,
{
    task::block_in_place(move || {
        let conn = repo.get_conn()?;
        let result = conn
            .pg_conn
            .build_transaction()
            .repeatable_read()
            .run(|| f(&conn));
        Ok(result?)
    })
}

pub fn get_pg_pool(database_url: String, max_size: u32) -> PgRepo {
    let manager = ConnectionManager::new(database_url.as_str());
    let pool = r2d2::Builder::default()
        .max_size(max_size)
        .build(manager)
        .expect("Could not instantiate db pool");
    let conn = pool.get().expect("Could not get db pool connection");
    embedded_migrations::run_with_output(&conn, &mut std::io::stdout())
        .expect("Could not run migrations");
    PgRepo { pool }
}

pub fn get_roll_back_err() -> diesel::result::Error {
    diesel::result::Error::RollbackTransaction
}
