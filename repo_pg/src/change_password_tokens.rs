use crate::errors::PgRepoError;
use crate::filters::GetAllChangePasswordTokensFilter;
use crate::models::{db_models, domain};
use crate::options::{ChangePasswordTokenSortType, PaginationOptions};
use crate::schema::change_password_tokens;
use crate::{change_sets::UpdateChangePasswordToken, insertables::NewChangePasswordToken};
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};

pub struct ChangePasswordTokenRepo<'a> {
    conn: &'a crate::pg_util::RepoConnection,
}

impl<'a> ChangePasswordTokenRepo<'a> {
    pub fn new(conn: &'a crate::pg_util::RepoConnection) -> Self {
        Self { conn }
    }

    pub fn insert_one(
        &self,
        new_verify_email_token: NewChangePasswordToken,
    ) -> Result<domain::ChangePasswordToken, diesel::result::Error> {
        let conn = &self.conn.pg_conn;
        let query =
            diesel::insert_into(change_password_tokens::table).values(&new_verify_email_token);
        let result = query.get_result(conn)?;
        Ok(domain::ChangePasswordToken::from(result))
    }

    pub fn update_one(
        &self,
        id_value: i32,
        updated_verify_email_token: UpdateChangePasswordToken,
    ) -> Result<domain::ChangePasswordToken, diesel::result::Error> {
        use crate::schema::change_password_tokens::dsl::{change_password_tokens, id};
        let conn = &self.conn.pg_conn;
        let query = diesel::update(change_password_tokens.filter(id.eq(id_value)))
            .set(&updated_verify_email_token);
        let result = query.get_result(conn)?;
        Ok(domain::ChangePasswordToken::from(result))
    }

    pub fn delete_one(&self, id_value: i32) -> Result<usize, diesel::result::Error> {
        use crate::schema::change_password_tokens::dsl::{change_password_tokens, id};
        let conn = &self.conn.pg_conn;
        let query = diesel::delete(change_password_tokens.filter(id.eq(id_value)));
        Ok(query.execute(conn)?)
    }

    pub fn find_one(
        &self,
        id_value: i32,
    ) -> Result<Option<domain::ChangePasswordToken>, diesel::result::Error> {
        use crate::schema::change_password_tokens::dsl::{change_password_tokens, id};

        let conn = &self.conn.pg_conn;
        let query = change_password_tokens
            .filter(id.eq(id_value))
            .select(change_password_tokens::all_columns());
        let verify_email_token: db_models::ChangePasswordToken =
            match query.first(conn).optional()? {
                Some(value) => value,
                None => return Ok(None),
            };
        Ok(Some(domain::ChangePasswordToken::from(verify_email_token)))
    }

    pub fn find_one_by_token(
        &self,
        token_value: String,
    ) -> Result<Option<domain::ChangePasswordToken>, diesel::result::Error> {
        use crate::schema::change_password_tokens::dsl::{change_password_tokens, token};

        let conn = &self.conn.pg_conn;
        let query = change_password_tokens
            .filter(token.eq(token_value))
            .select(change_password_tokens::all_columns());
        let verify_email_token: db_models::ChangePasswordToken =
            match query.first(conn).optional()? {
                Some(value) => value,
                None => return Ok(None),
            };
        Ok(Some(domain::ChangePasswordToken::from(verify_email_token)))
    }

    pub fn find(
        &self,
        filter: GetAllChangePasswordTokensFilter,
        sort: Option<ChangePasswordTokenSortType>,
        pagination: PaginationOptions,
    ) -> Result<Vec<domain::ChangePasswordToken>, diesel::result::Error> {
        use crate::schema::change_password_tokens::dsl::change_password_tokens;
        let q = change_password_tokens
            .select(change_password_tokens::all_columns())
            .into_boxed();

        let q = if let (Some(page), Some(page_size)) = (pagination.page, pagination.page_size) {
            q.offset((page - 1) * page_size).limit(page_size)
        } else {
            q
        };

        let conn = &self.conn.pg_conn;
        let results: Vec<db_models::ChangePasswordToken> = q.load(conn)?;

        Ok(results
            .into_iter()
            .map(|verify_email_token| domain::ChangePasswordToken::from(verify_email_token))
            .collect::<Vec<_>>())
    }
}
