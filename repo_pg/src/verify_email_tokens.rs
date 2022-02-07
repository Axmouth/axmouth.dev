use crate::entity::verify_email_tokens::{
    ActiveModel as VerifyEmailTokenActiveModel, Column as VerifyEmailTokenColumn,
    Entity as VerifyEmailTokenEntity,
};
use crate::errors::PgRepoError;
use crate::filters::GetAllVerifyEmailTokensFilter;
use crate::models::domain;
use crate::options::{PaginationOptions, VerifyEmailTokenSortType};
use crate::{change_sets::UpdateVerifyEmailToken, insertables::NewVerifyEmailToken};
use sea_orm::{prelude::*, ActiveValue, QueryOrder, QuerySelect};

pub struct VerifyEmailTokenRepo<'a> {
    conn: &'a DatabaseConnection,
}

impl<'a> VerifyEmailTokenRepo<'a> {
    pub fn new(conn: &'a DatabaseConnection) -> Self {
        Self { conn }
    }

    pub async fn insert_one(
        &self,
        new_verify_email_token: NewVerifyEmailToken,
    ) -> Result<domain::VerifyEmailToken, PgRepoError> {
        let verify_email_token: VerifyEmailTokenActiveModel = new_verify_email_token.into();
        Ok(verify_email_token.insert(self.conn).await.map(From::from)?)
    }

    pub async fn update_one(
        &self,
        id_value: i32,
        updated_verify_email_token: UpdateVerifyEmailToken,
    ) -> Result<domain::VerifyEmailToken, PgRepoError> {
        let mut verify_email_token: VerifyEmailTokenActiveModel = updated_verify_email_token.into();
        verify_email_token.id = ActiveValue::Unchanged(id_value);
        let result = verify_email_token.update(self.conn).await?;
        Ok(result.into())
    }

    pub async fn delete_one(&self, id_value: i32) -> Result<u64, PgRepoError> {
        let token = VerifyEmailTokenEntity::find()
            .filter(VerifyEmailTokenColumn::Id.eq(id_value))
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
        id_value: i32,
    ) -> Result<Option<domain::VerifyEmailToken>, PgRepoError> {
        Ok(VerifyEmailTokenEntity::find()
            .filter(VerifyEmailTokenColumn::Id.eq(id_value))
            .one(self.conn)
            .await?
            .map(From::from))
    }

    pub async fn find_one_by_token(
        &self,
        token_value: String,
    ) -> Result<Option<domain::VerifyEmailToken>, PgRepoError> {
        Ok(VerifyEmailTokenEntity::find()
            .filter(VerifyEmailTokenColumn::Token.eq(token_value))
            .one(self.conn)
            .await?
            .map(From::from))
    }

    pub async fn find(
        &self,
        _: GetAllVerifyEmailTokensFilter,
        _: Option<VerifyEmailTokenSortType>,
        pagination: PaginationOptions,
    ) -> Result<Vec<domain::VerifyEmailToken>, PgRepoError> {
        let q = VerifyEmailTokenEntity::find().order_by_asc(VerifyEmailTokenColumn::CreatedAt);

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
