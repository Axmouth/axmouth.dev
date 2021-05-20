use crate::filters::GetAllChangePasswordTokensFilter;
use crate::models::{db_models, domain};
use crate::options::{ChangePasswordTokenSortType, PaginationOptions};
use crate::schema::change_password_tokens;
use crate::{change_sets::UpdateChangePasswordToken, insertables::NewChangePasswordToken};
use crate::{errors::PgRepoError, pg_util::Repo};
use diesel::prelude::*;
use diesel::{r2d2::ConnectionManager, PgConnection, QueryDsl, RunQueryDsl};
use r2d2::Pool;

#[derive(Clone)]
pub struct ChangePasswordTokenRepo {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl ChangePasswordTokenRepo {
    pub fn new(repo: Repo) -> Self {
        Self { pool: repo.pool }
    }

    pub async fn insert_one(
        &self,
        new_verify_email_token: NewChangePasswordToken,
    ) -> Result<domain::ChangePasswordToken, PgRepoError> {
        let conn = self.pool.get()?;
        let query =
            diesel::insert_into(change_password_tokens::table).values(&new_verify_email_token);
        let result = tokio::task::block_in_place(move || query.get_result(&conn))?;
        Ok(domain::ChangePasswordToken::from(result))
    }

    pub async fn update_one(
        &self,
        id_value: i32,
        updated_verify_email_token: UpdateChangePasswordToken,
    ) -> Result<domain::ChangePasswordToken, PgRepoError> {
        use crate::schema::change_password_tokens::dsl::{change_password_tokens, id};
        let conn = self.pool.get()?;
        let query = diesel::update(change_password_tokens.filter(id.eq(id_value)))
            .set(&updated_verify_email_token);
        let result = tokio::task::block_in_place(move || query.get_result(&conn))?;
        Ok(domain::ChangePasswordToken::from(result))
    }

    pub async fn delete_one(&self, id_value: i32) -> Result<usize, PgRepoError> {
        use crate::schema::change_password_tokens::dsl::{change_password_tokens, id};
        let conn = self.pool.get()?;
        let query = diesel::delete(change_password_tokens.filter(id.eq(id_value)));
        Ok(query.execute(&conn)?)
    }

    pub async fn find_one(
        &self,
        id_value: i32,
    ) -> Result<Option<domain::ChangePasswordToken>, PgRepoError> {
        use crate::schema::change_password_tokens::dsl::{change_password_tokens, id};

        let conn = self.pool.get()?;
        let query = change_password_tokens
            .filter(id.eq(id_value))
            .select(change_password_tokens::all_columns());
        let verify_email_token: db_models::ChangePasswordToken =
            match tokio::task::block_in_place(move || query.first(&conn).optional())? {
                Some(value) => value,
                None => return Ok(None),
            };
        Ok(Some(domain::ChangePasswordToken::from(verify_email_token)))
    }

    pub async fn find_one_by_token(
        &self,
        token_value: String,
    ) -> Result<Option<domain::ChangePasswordToken>, PgRepoError> {
        use crate::schema::change_password_tokens::dsl::{change_password_tokens, token};

        let conn = self.pool.get()?;
        let query = change_password_tokens
            .filter(token.eq(token_value))
            .select(change_password_tokens::all_columns());
        let verify_email_token: db_models::ChangePasswordToken =
            match tokio::task::block_in_place(move || query.first(&conn).optional())? {
                Some(value) => value,
                None => return Ok(None),
            };
        Ok(Some(domain::ChangePasswordToken::from(verify_email_token)))
    }

    pub async fn find(
        &self,
        filter: GetAllChangePasswordTokensFilter,
        sort: Option<ChangePasswordTokenSortType>,
        pagination: PaginationOptions,
    ) -> Result<Vec<domain::ChangePasswordToken>, PgRepoError> {
        use crate::schema::change_password_tokens::dsl::change_password_tokens;
        let q = change_password_tokens
            .select(change_password_tokens::all_columns())
            .into_boxed();

        let q = if let (Some(page), Some(page_size)) = (pagination.page, pagination.page_size) {
            q.offset((page - 1) * page_size).limit(page_size)
        } else {
            q
        };

        let conn = self.pool.get()?;
        let results: Vec<db_models::ChangePasswordToken> =
            tokio::task::block_in_place(move || q.load(&conn))?;

        Ok(results
            .into_iter()
            .map(|verify_email_token| domain::ChangePasswordToken::from(verify_email_token))
            .collect::<Vec<_>>())
    }
}
