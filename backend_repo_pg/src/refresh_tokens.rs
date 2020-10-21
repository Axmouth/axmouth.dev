use crate::models::{db_models, domain};
use crate::schema::refresh_tokens;
use crate::{
    change_sets::UpdateRefreshToken, filters::GetAllRefreshTokensFilter,
    insertables::NewRefreshToken, options::PaginationOptions,
};
use crate::{errors::PgRepoError, options::RefreshTokenSortType};
use diesel::prelude::*;
use diesel::{r2d2::ConnectionManager, PgConnection, QueryDsl, RunQueryDsl};
use r2d2::Pool;

#[derive(Clone)]
pub struct RefreshTokenRepo {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl RefreshTokenRepo {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }

    pub async fn insert_one(
        &self,
        new_token: NewRefreshToken,
    ) -> Result<db_models::RefreshToken, PgRepoError> {
        let conn = self.pool.get()?;
        let query = diesel::insert_into(refresh_tokens::table).values(&new_token);
        Ok(tokio::task::block_in_place(move || {
            query.get_result(&conn)
        })?)
    }

    pub async fn update_one(
        &self,
        id_value: uuid::Uuid,
        updated_token: UpdateRefreshToken,
    ) -> Result<db_models::RefreshToken, PgRepoError> {
        use crate::schema::refresh_tokens::dsl::{id, refresh_tokens};
        let conn = self.pool.get()?;
        let query = diesel::update(refresh_tokens.filter(id.eq(id_value))).set(&updated_token);
        Ok(tokio::task::block_in_place(move || {
            query.get_result(&conn)
        })?)
    }

    pub async fn use_up(
        &self,
        id_value: uuid::Uuid,
    ) -> Result<db_models::RefreshToken, PgRepoError> {
        self.update_one(
            id_value,
            UpdateRefreshToken {
                invalidated: None,
                used: Some(true),
            },
        )
        .await
    }

    pub async fn invalidate(
        &self,
        id_value: uuid::Uuid,
    ) -> Result<db_models::RefreshToken, PgRepoError> {
        self.update_one(
            id_value,
            UpdateRefreshToken {
                invalidated: Some(true),
                used: None,
            },
        )
        .await
    }

    pub async fn delete_one(&self, id_value: uuid::Uuid) -> Result<usize, PgRepoError> {
        use crate::schema::refresh_tokens::dsl::{id, refresh_tokens};
        let conn = self.pool.get()?;
        let query = diesel::delete(refresh_tokens.filter(id.eq(id_value)));
        Ok(tokio::task::block_in_place(move || query.execute(&conn))?)
    }

    pub async fn find_one(
        &self,
        id_value: uuid::Uuid,
    ) -> Result<Option<domain::RefreshToken>, PgRepoError> {
        use crate::schema::refresh_tokens::dsl::{id, refresh_tokens};

        let conn = self.pool.get()?;
        let query = refresh_tokens
            .filter(id.eq(id_value))
            .select(refresh_tokens::all_columns());
        let token: db_models::RefreshToken =
            match tokio::task::block_in_place(move || query.first(&conn).optional())? {
                Some(value) => value,
                None => return Ok(None),
            };
        Ok(Some(domain::RefreshToken::from(token)))
    }

    pub async fn find(
        &self,
        filter: GetAllRefreshTokensFilter,
        sort: Option<RefreshTokenSortType>,
        pagination: PaginationOptions,
    ) -> Result<Vec<domain::RefreshToken>, PgRepoError> {
        use crate::schema::refresh_tokens::dsl::refresh_tokens;
        let q = refresh_tokens
            .select(refresh_tokens::all_columns())
            .into_boxed();

        let q = if let (Some(page), Some(page_size)) = (pagination.page, pagination.page_size) {
            q.offset((page - 1) * page_size).limit(page_size)
        } else {
            q
        };

        let conn = self.pool.get()?;
        let results: Vec<db_models::RefreshToken> =
            tokio::task::block_in_place(move || q.load(&conn))?;

        Ok(results
            .into_iter()
            .map(|token| domain::RefreshToken::from(token))
            .collect::<Vec<_>>())
    }
}
