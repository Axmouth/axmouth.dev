use crate::models::db_models;
use crate::{errors::PgRepoError, pg_util::Repo};
use diesel::prelude::*;
use diesel::{r2d2::ConnectionManager, PgConnection, QueryDsl, RunQueryDsl};
use r2d2::Pool;

#[derive(Clone)]
pub struct HealthRepo {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl HealthRepo {
    pub fn new(repo: Repo) -> Self {
        Self { pool: repo.pool }
    }

    pub async fn check(&self) -> Result<Option<u128>, PgRepoError> {
        use crate::schema::users::dsl::users;

        let start = tokio::time::Instant::now();
        let conn = self.pool.get()?;
        let query = users.select(users::all_columns());
        let _: db_models::User =
            match tokio::task::block_in_place(move || query.first(&conn).optional())? {
                Some(value) => value,
                None => return Ok(None),
            };
        let duration = start.elapsed();
        Ok(Some(duration.as_millis()))
    }
}
