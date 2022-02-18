use crate::entity::change_password_tokens::{
    ActiveModel as ChangePasswordTokenActiveModel, Column as ChangePasswordTokenColumn,
    Entity as ChangePasswordTokenEntity,
};
use crate::errors::PgRepoError;
use crate::filters::GetAllChangePasswordTokensFilter;
use crate::models::domain;
use crate::options::{ChangePasswordTokenSortType, PaginationOptions};
use crate::{change_sets::UpdateChangePasswordToken, insertables::NewChangePasswordToken};
use sea_orm::{prelude::*, ActiveValue, QueryOrder, QuerySelect};

pub struct ChangePasswordTokenRepo<'a> {
    conn: &'a DatabaseConnection,
}

impl<'a> ChangePasswordTokenRepo<'a> {
    pub fn new(conn: &'a DatabaseConnection) -> Self {
        Self { conn }
    }

    pub async fn insert_one(
        &self,
        new_verify_email_token: NewChangePasswordToken,
    ) -> Result<domain::ChangePasswordToken, PgRepoError> {
        let verify_email_token: ChangePasswordTokenActiveModel = new_verify_email_token.into();
        Ok(verify_email_token.insert(self.conn).await.map(From::from)?)
    }

    pub async fn update_one(
        &self,
        id_value: i32,
        updated_verify_email_token: UpdateChangePasswordToken,
    ) -> Result<domain::ChangePasswordToken, PgRepoError> {
        let mut verify_email_token: ChangePasswordTokenActiveModel =
            updated_verify_email_token.into();
        verify_email_token.id = ActiveValue::Unchanged(id_value);
        let result = verify_email_token.update(self.conn).await?;
        Ok(result.into())
    }

    pub async fn delete_one(&self, id_value: i32) -> Result<u64, PgRepoError> {
        let user = ChangePasswordTokenEntity::find()
            .filter(ChangePasswordTokenColumn::Id.eq(id_value))
            .one(self.conn)
            .await?;

        if let Some(user) = user {
            Ok(user.delete(self.conn).await?.rows_affected)
        } else {
            Ok(0)
        }
    }

    pub async fn find_one(
        &self,
        id_value: i32,
    ) -> Result<Option<domain::ChangePasswordToken>, PgRepoError> {
        Ok(ChangePasswordTokenEntity::find()
            .filter(ChangePasswordTokenColumn::Id.eq(id_value))
            .one(self.conn)
            .await?
            .map(From::from))
    }

    pub async fn find_one_by_token(
        &self,
        token_value: String,
    ) -> Result<Option<domain::ChangePasswordToken>, PgRepoError> {
        Ok(ChangePasswordTokenEntity::find()
            .filter(ChangePasswordTokenColumn::Token.eq(token_value))
            .one(self.conn)
            .await?
            .map(From::from))
    }

    pub async fn find(
        &self,
        _: GetAllChangePasswordTokensFilter,
        _: Option<ChangePasswordTokenSortType>,
        pagination: PaginationOptions,
    ) -> Result<Vec<domain::ChangePasswordToken>, PgRepoError> {
        let q =
            ChangePasswordTokenEntity::find().order_by_asc(ChangePasswordTokenColumn::CreatedAt);

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
