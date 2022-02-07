use crate::entity::refresh_tokens::{
    ActiveModel as RefreshTokenActiveModel, Column as RefreshTokenColumn,
    Entity as RefreshTokenEntity,
};
use crate::models::domain;
use crate::{
    change_sets::UpdateRefreshToken, filters::GetAllRefreshTokensFilter,
    insertables::NewRefreshToken, options::PaginationOptions,
};
use crate::{errors::PgRepoError, options::RefreshTokenSortType};
use sea_orm::{prelude::*, ActiveValue, QueryOrder, QuerySelect};

pub struct RefreshTokenRepo<'a> {
    conn: &'a DatabaseConnection,
}

impl<'a> RefreshTokenRepo<'a> {
    pub fn new(conn: &'a DatabaseConnection) -> Self {
        Self { conn }
    }

    pub async fn insert_one(
        &self,
        new_token: NewRefreshToken,
    ) -> Result<domain::RefreshToken, PgRepoError> {
        let token: RefreshTokenActiveModel = new_token.into();
        Ok(token.insert(self.conn).await.map(From::from)?)
    }

    pub async fn update_one(
        &self,
        id_value: uuid::Uuid,
        updated_token: UpdateRefreshToken,
    ) -> Result<domain::RefreshToken, PgRepoError> {
        let mut token: RefreshTokenActiveModel = updated_token.into();
        token.id = ActiveValue::Unchanged(id_value);
        let result = token.update(self.conn).await?;
        Ok(result.into())
    }

    pub async fn use_up(&self, id_value: uuid::Uuid) -> Result<domain::RefreshToken, PgRepoError> {
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
    ) -> Result<domain::RefreshToken, PgRepoError> {
        self.update_one(
            id_value,
            UpdateRefreshToken {
                invalidated: Some(true),
                used: None,
            },
        )
        .await
    }

    pub async fn delete_one(&self, id_value: uuid::Uuid) -> Result<u64, PgRepoError> {
        let token = RefreshTokenEntity::find()
            .filter(RefreshTokenColumn::Id.eq(id_value))
            .one(self.conn)
            .await?;

        if let Some(token) = token {
            Ok(token.delete(self.conn).await?.rows_affected)
        } else {
            Ok(0)
        }
    }

    pub async fn find_one(
        &self,
        id_value: uuid::Uuid,
    ) -> Result<Option<domain::RefreshToken>, PgRepoError> {
        Ok(RefreshTokenEntity::find()
            .filter(RefreshTokenColumn::Id.eq(id_value))
            .one(self.conn)
            .await?
            .map(From::from))
    }

    pub async fn find(
        &self,
        _: GetAllRefreshTokensFilter,
        _: Option<RefreshTokenSortType>,
        pagination: PaginationOptions,
    ) -> Result<Vec<domain::RefreshToken>, PgRepoError> {
        let q = RefreshTokenEntity::find().order_by_asc(RefreshTokenColumn::CreatedAt);

        let q = if let (Some(page), Some(page_size)) = (pagination.page, pagination.page_size) {
            q.offset(((page - 1) * page_size) as u64)
                .limit(page_size as u64)
        } else {
            q
        };

        let results = q.all(self.conn).await?;

        Ok(results.into_iter().map(From::from).collect::<Vec<_>>())
    }
}
