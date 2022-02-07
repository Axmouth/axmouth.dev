extern crate diesel;
extern crate dotenv;

embed_migrations!();

use std::{sync::Arc, time::Duration};

use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::QueryResult;
use r2d2::{Pool, PooledConnection};
use sea_orm::{
    ConnectOptions, Database, DatabaseConnection, DatabaseTransaction, TransactionTrait,
};
use tokio::task;

use crate::errors::PgRepoError;

pub trait Repo {
    fn get_pool_connection(
        &self,
    ) -> Result<PooledConnection<ConnectionManager<PgConnection>>, PgRepoError>;

    fn get_pool(&self) -> Pool<ConnectionManager<PgConnection>>;

    fn get_async_pool(&self) -> DatabaseConnection;

    fn get_conn(&self) -> Result<RepoConnection, PgRepoError>;

    fn get_async_conn(&self) -> DatabaseConnection;
}

#[derive(Clone)]
pub struct PgRepo {
    pub(crate) pool: Pool<ConnectionManager<PgConnection>>,
    pub(crate) async_pool: DatabaseConnection,
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

    fn get_async_pool(&self) -> DatabaseConnection {
        self.async_pool.clone()
    }

    fn get_conn(&self) -> Result<RepoConnection, PgRepoError> {
        Ok(RepoConnection {
            pg_conn: self.pool.get()?,
        })
    }

    fn get_async_conn(&self) -> DatabaseConnection {
        self.async_pool.clone()
    }
}

pub struct RepoConnection {
    pub(crate) pg_conn: PooledConnection<ConnectionManager<PgConnection>>,
}

impl PgRepo {
    pub fn new(
        pool: Pool<ConnectionManager<PgConnection>>,
        async_pool: DatabaseConnection,
    ) -> Self {
        Self { pool, async_pool }
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

pub async fn begin_pg_async_transaction(
    db: DatabaseConnection,
) -> Result<DatabaseTransaction, PgRepoError> {
    Ok(db.begin().await?)
}

pub async fn commit_pg_async_transaction<ResultType, Func>(
    db: DatabaseTransaction,
) -> Result<(), PgRepoError> {
    Ok(db.commit().await?)
}

pub async fn get_pg_pool(database_url: String, max_size: u32) -> PgRepo {
    let manager = ConnectionManager::new(database_url.as_str());
    let pool = r2d2::Builder::default()
        .max_size(max_size)
        .build(manager)
        .expect("Could not instantiate db pool");
    let conn = pool.get().expect("Could not get db pool connection");
    embedded_migrations::run_with_output(&conn, &mut std::io::stdout())
        .expect("Could not run migrations");

    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(32)
        .min_connections(2)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .sqlx_logging(true);

    let async_pool = Database::connect(opt)
        .await
        .expect("Could not get async db pool connection");

    PgRepo { pool, async_pool }
}

pub fn get_roll_back_err() -> diesel::result::Error {
    diesel::result::Error::RollbackTransaction
}
