use crate::filters::GetAllVerifyEmailTokensFilter;
use crate::models::{db_models, domain};
use crate::options::{PaginationOptions, VerifyEmailTokenSortType};
use crate::schema::verify_email_tokens;
use crate::{change_sets::UpdateVerifyEmailToken, insertables::NewVerifyEmailToken};
use crate::{errors::PgRepoError, pg_util::Repo};
use diesel::prelude::*;
use diesel::{r2d2::ConnectionManager, PgConnection, QueryDsl, RunQueryDsl};
use r2d2::Pool;

#[derive(Clone)]
pub struct VerifyEmailTokenRepo {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl VerifyEmailTokenRepo {
    pub fn new(repo: Repo) -> Self {
        Self { pool: repo.pool }
    }

    pub async fn insert_one(
        &self,
        new_verify_email_token: NewVerifyEmailToken,
    ) -> Result<domain::VerifyEmailToken, PgRepoError> {
        let conn = self.pool.get()?;
        let query = diesel::insert_into(verify_email_tokens::table).values(&new_verify_email_token);
        let result = tokio::task::block_in_place(move || query.get_result(&conn))?;
        Ok(domain::VerifyEmailToken::from(result))
    }

    pub async fn update_one(
        &self,
        id_value: i32,
        updated_verify_email_token: UpdateVerifyEmailToken,
    ) -> Result<domain::VerifyEmailToken, PgRepoError> {
        use crate::schema::verify_email_tokens::dsl::{id, verify_email_tokens};
        let conn = self.pool.get()?;
        let query = diesel::update(verify_email_tokens.filter(id.eq(id_value)))
            .set(&updated_verify_email_token);
        let result = tokio::task::block_in_place(move || query.get_result(&conn))?;
        Ok(domain::VerifyEmailToken::from(result))
    }

    pub async fn delete_one(&self, id_value: i32) -> Result<usize, PgRepoError> {
        use crate::schema::verify_email_tokens::dsl::{id, verify_email_tokens};
        let conn = self.pool.get()?;
        let query = diesel::delete(verify_email_tokens.filter(id.eq(id_value)));
        Ok(query.execute(&conn)?)
    }

    pub async fn find_one(
        &self,
        id_value: i32,
    ) -> Result<Option<domain::VerifyEmailToken>, PgRepoError> {
        use crate::schema::verify_email_tokens::dsl::{id, verify_email_tokens};

        let conn = self.pool.get()?;
        let query = verify_email_tokens
            .filter(id.eq(id_value))
            .select(verify_email_tokens::all_columns());
        let verify_email_token: db_models::VerifyEmailToken =
            match tokio::task::block_in_place(move || query.first(&conn).optional())? {
                Some(value) => value,
                None => return Ok(None),
            };
        Ok(Some(domain::VerifyEmailToken::from(verify_email_token)))
    }

    pub async fn find_one_by_token(
        &self,
        token_value: String,
    ) -> Result<Option<domain::VerifyEmailToken>, PgRepoError> {
        use crate::schema::verify_email_tokens::dsl::{token, verify_email_tokens};

        let conn = self.pool.get()?;
        let query = verify_email_tokens
            .filter(token.eq(token_value))
            .select(verify_email_tokens::all_columns());
        let verify_email_token: db_models::VerifyEmailToken =
            match tokio::task::block_in_place(move || query.first(&conn).optional())? {
                Some(value) => value,
                None => return Ok(None),
            };
        Ok(Some(domain::VerifyEmailToken::from(verify_email_token)))
    }

    pub async fn find(
        &self,
        filter: GetAllVerifyEmailTokensFilter,
        sort: Option<VerifyEmailTokenSortType>,
        pagination: PaginationOptions,
    ) -> Result<Vec<domain::VerifyEmailToken>, PgRepoError> {
        use crate::schema::verify_email_tokens::dsl::verify_email_tokens;
        let q = verify_email_tokens
            .select(verify_email_tokens::all_columns())
            .into_boxed();

        let q = if let (Some(page), Some(page_size)) = (pagination.page, pagination.page_size) {
            q.offset((page - 1) * page_size).limit(page_size)
        } else {
            q
        };

        let conn = self.pool.get()?;
        let results: Vec<db_models::VerifyEmailToken> =
            tokio::task::block_in_place(move || q.load(&conn))?;

        Ok(results
            .into_iter()
            .map(|verify_email_token| domain::VerifyEmailToken::from(verify_email_token))
            .collect::<Vec<_>>())
    }
}
