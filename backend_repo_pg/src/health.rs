use crate::errors::PgRepoError;
use crate::models::db_models;
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};

pub struct HealthRepo<'a> {
    conn: &'a crate::pg_util::RepoConnection,
}

impl<'a> HealthRepo<'a> {
    pub fn new(conn: &'a crate::pg_util::RepoConnection) -> Self {
        Self { conn }
    }

    pub fn check(&self) -> Result<Option<u128>, diesel::result::Error> {
        use crate::schema::users::dsl::users;

        let start = tokio::time::Instant::now();
        let conn = &self.conn.pg_conn;
        let query = users.select(users::all_columns());
        let _: db_models::User = match query.first(conn).optional()? {
            Some(value) => value,
            None => return Ok(None),
        };
        let duration = start.elapsed();
        Ok(Some(duration.as_millis()))
    }
}
