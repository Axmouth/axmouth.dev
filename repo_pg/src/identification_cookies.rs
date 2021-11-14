use crate::errors::PgRepoError;
use crate::filters::GetAllIdentificationCookiesFilter;
use crate::insertables::NewIdentificationCookie;
use crate::models::{db_models, domain};
use crate::options::{IdentificationCookieSortType, PaginationOptions};
use crate::schema::identification_cookies;
use chrono::Utc;
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};

pub struct IdentificationCookieRepo<'a> {
    conn: &'a crate::pg_util::RepoConnection,
}

impl<'a> IdentificationCookieRepo<'a> {
    pub fn new(conn: &'a crate::pg_util::RepoConnection) -> Self {
        Self { conn }
    }

    pub fn insert_one(
        &self,
        new_text_body: NewIdentificationCookie,
    ) -> Result<domain::IdentificationCookie, diesel::result::Error> {
        let conn = &self.conn.pg_conn;
        let query = diesel::insert_into(identification_cookies::table).values(&new_text_body);
        let result = query.get_result(conn)?;
        Ok(domain::IdentificationCookie::from(result))
    }

    pub fn delete_one(&self, id_value: i32) -> Result<usize, diesel::result::Error> {
        use crate::schema::identification_cookies::dsl::{id, identification_cookies};
        let conn = &self.conn.pg_conn;
        let query = diesel::delete(identification_cookies.filter(id.eq(id_value)));
        Ok(query.execute(conn)?)
    }

    pub fn find_one(
        &self,
        id_value: i32,
    ) -> Result<Option<domain::IdentificationCookie>, diesel::result::Error> {
        use crate::schema::identification_cookies::dsl::{id, identification_cookies};

        let conn = &self.conn.pg_conn;
        let query = identification_cookies
            .filter(id.eq(id_value))
            .select(identification_cookies::all_columns());
        let text_body: db_models::IdentificationCookie = match query.first(conn).optional()? {
            Some(value) => value,
            None => return Ok(None),
        };
        Ok(Some(domain::IdentificationCookie::from(text_body)))
    }

    pub fn find_one_by_token(
        &self,
        token_value: &str,
    ) -> Result<Option<domain::IdentificationCookie>, diesel::result::Error> {
        use crate::schema::identification_cookies::dsl::{
            expires_at, identification_cookies, token,
        };

        let conn = &self.conn.pg_conn;
        let query = identification_cookies
            .filter(expires_at.gt(Utc::now().naive_utc()))
            .filter(token.eq(token_value))
            .select(identification_cookies::all_columns());
        let text_body: db_models::IdentificationCookie = match query.first(conn).optional()? {
            Some(value) => value,
            None => return Ok(None),
        };
        Ok(Some(domain::IdentificationCookie::from(text_body)))
    }

    pub fn find_one_by_hash(
        &self,
        hash_value: String,
    ) -> Result<Option<domain::IdentificationCookie>, diesel::result::Error> {
        use crate::schema::identification_cookies::dsl::{
            expires_at, id_hash, identification_cookies,
        };

        let conn = &self.conn.pg_conn;
        let query = identification_cookies
            .filter(expires_at.gt(Utc::now().naive_utc()))
            .filter(id_hash.eq(hash_value))
            .select(identification_cookies::all_columns());
        let text_body: db_models::IdentificationCookie = match query.first(conn).optional()? {
            Some(value) => value,
            None => return Ok(None),
        };
        Ok(Some(domain::IdentificationCookie::from(text_body)))
    }

    pub fn find(
        &self,
        filter: GetAllIdentificationCookiesFilter,
        sort: Option<IdentificationCookieSortType>,
        pagination: PaginationOptions,
    ) -> Result<Vec<domain::IdentificationCookie>, diesel::result::Error> {
        use crate::schema::identification_cookies::dsl::identification_cookies;
        let q = identification_cookies
            .select(identification_cookies::all_columns())
            .into_boxed();

        let q = if let (Some(page), Some(page_size)) = (pagination.page, pagination.page_size) {
            q.offset((page - 1) * page_size).limit(page_size)
        } else {
            q
        };

        let conn = &self.conn.pg_conn;
        let results: Vec<db_models::IdentificationCookie> = q.load(conn)?;

        Ok(results
            .into_iter()
            .map(|text_body| domain::IdentificationCookie::from(text_body))
            .collect::<Vec<_>>())
    }
}
