use crate::models::{db_models, domain};
use crate::schema::refresh_tokens;
use crate::{
    change_sets::UpdateRefreshToken, filters::GetAllRefreshTokensFilter,
    insertables::NewRefreshToken, options::PaginationOptions,
};
use crate::{errors::PgRepoError, options::RefreshTokenSortType};
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};

pub struct RefreshTokenRepo<'a> {
    conn: &'a crate::pg_util::RepoConnection,
}

impl<'a> RefreshTokenRepo<'a> {
    pub fn new(conn: &'a crate::pg_util::RepoConnection) -> Self {
        Self { conn }
    }

    pub fn insert_one(
        &self,
        new_token: NewRefreshToken,
    ) -> Result<db_models::RefreshToken, diesel::result::Error> {
        let conn = &self.conn.pg_conn;
        let query = diesel::insert_into(refresh_tokens::table).values(&new_token);
        Ok(query.get_result(conn)?)
    }

    pub fn update_one(
        &self,
        id_value: uuid::Uuid,
        updated_token: UpdateRefreshToken,
    ) -> Result<db_models::RefreshToken, diesel::result::Error> {
        use crate::schema::refresh_tokens::dsl::{id, refresh_tokens};
        let conn = &self.conn.pg_conn;
        let query = diesel::update(refresh_tokens.filter(id.eq(id_value))).set(&updated_token);
        Ok(query.get_result(conn)?)
    }

    pub fn use_up(
        &self,
        id_value: uuid::Uuid,
    ) -> Result<db_models::RefreshToken, diesel::result::Error> {
        self.update_one(
            id_value,
            UpdateRefreshToken {
                invalidated: None,
                used: Some(true),
            },
        )
    }

    pub fn invalidate(
        &self,
        id_value: uuid::Uuid,
    ) -> Result<db_models::RefreshToken, diesel::result::Error> {
        self.update_one(
            id_value,
            UpdateRefreshToken {
                invalidated: Some(true),
                used: None,
            },
        )
    }

    pub fn delete_one(&self, id_value: uuid::Uuid) -> Result<usize, diesel::result::Error> {
        use crate::schema::refresh_tokens::dsl::{id, refresh_tokens};
        let conn = &self.conn.pg_conn;
        let query = diesel::delete(refresh_tokens.filter(id.eq(id_value)));
        Ok(query.execute(conn)?)
    }

    pub fn find_one(
        &self,
        id_value: uuid::Uuid,
    ) -> Result<Option<domain::RefreshToken>, diesel::result::Error> {
        use crate::schema::refresh_tokens::dsl::{id, refresh_tokens};

        let conn = &self.conn.pg_conn;
        let query = refresh_tokens
            .filter(id.eq(id_value))
            .select(refresh_tokens::all_columns());
        let token: db_models::RefreshToken = match query.first(conn).optional()? {
            Some(value) => value,
            None => return Ok(None),
        };
        Ok(Some(domain::RefreshToken::from(token)))
    }

    pub fn find(
        &self,
        filter: GetAllRefreshTokensFilter,
        sort: Option<RefreshTokenSortType>,
        pagination: PaginationOptions,
    ) -> Result<Vec<domain::RefreshToken>, diesel::result::Error> {
        use crate::schema::refresh_tokens::dsl::refresh_tokens;
        let q = refresh_tokens
            .select(refresh_tokens::all_columns())
            .into_boxed();

        let q = if let (Some(page), Some(page_size)) = (pagination.page, pagination.page_size) {
            q.offset((page - 1) * page_size).limit(page_size)
        } else {
            q
        };

        let conn = &self.conn.pg_conn;
        let results: Vec<db_models::RefreshToken> = q.load(conn)?;

        Ok(results
            .into_iter()
            .map(|token| domain::RefreshToken::from(token))
            .collect::<Vec<_>>())
    }
}
