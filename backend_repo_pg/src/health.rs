use crate::models::{db_models, domain};
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

    pub async fn check(&self, id_value: i32) -> Result<Option<domain::User>, PgRepoError> {
        use crate::schema::users::dsl::{id, users};

        let conn = self.pool.get()?;
        let query = users.filter(id.eq(id_value)).select(users::all_columns());
        let user: db_models::User =
            match tokio::task::block_in_place(move || query.first(&conn).optional())? {
                Some(value) => value,
                None => return Ok(None),
            };
        Ok(Some(domain::User::from(user)))
    }
}
