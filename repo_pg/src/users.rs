use crate::entity::users::{
    ActiveModel as UserActiveModel, Column as UserColumn, Entity as UserEntity, Model as UserModel,
};
use crate::errors::PgRepoError;
use crate::filters::GetAllUsersFilter;
use crate::models::domain;
use crate::options::{PaginationOptions, UserSortType};
use crate::{change_sets::UpdateUser, insertables::NewUser};
use sea_orm::{prelude::*, ActiveValue, QueryOrder, QuerySelect};

pub struct UserRepo<'a> {
    conn: &'a DatabaseConnection,
}

impl<'a> UserRepo<'a> {
    pub fn new(conn: &'a DatabaseConnection) -> Self {
        Self { conn }
    }

    pub async fn insert_one(&self, new_user: NewUser) -> Result<domain::User, PgRepoError> {
        let user: UserActiveModel = new_user.into();
        Ok(user.insert(self.conn).await.map(From::from)?)
    }

    pub async fn update_one(
        &self,
        id_value: i32,
        updated_user: UpdateUser,
    ) -> Result<domain::User, PgRepoError> {
        let mut user: UserActiveModel = updated_user.into();
        user.id = ActiveValue::Unchanged(id_value);
        let result = user.update(self.conn).await?;
        Ok(result.into())
    }

    pub async fn delete_one(&self, id_value: i32) -> Result<u64, PgRepoError> {
        let user = UserEntity::find()
            .filter(UserColumn::Id.eq(id_value))
            .one(self.conn)
            .await?;

        if let Some(user) = user {
            Ok(user.delete(self.conn).await?.rows_affected)
        } else {
            Ok(0)
        }
    }

    pub async fn find_one(&self, id_value: i32) -> Result<Option<UserModel>, PgRepoError> {
        Ok(UserEntity::find()
            .filter(UserColumn::Id.eq(id_value))
            .one(self.conn)
            .await?)
    }

    pub async fn find_one_by_email(
        &self,
        email_value: String,
    ) -> Result<Option<UserModel>, PgRepoError> {
        Ok(UserEntity::find()
            .filter(UserColumn::Email.eq(email_value))
            .one(self.conn)
            .await?)
    }

    pub async fn find_one_by_display_name(
        &self,
        display_name_value: String,
    ) -> Result<Option<UserModel>, PgRepoError> {
        Ok(UserEntity::find()
            .filter(UserColumn::DisplayName.eq(display_name_value))
            .one(self.conn)
            .await?)
    }

    pub async fn find(
        &self,
        _: GetAllUsersFilter,
        _: Option<UserSortType>,
        pagination: PaginationOptions,
    ) -> Result<Vec<domain::User>, PgRepoError> {
        let q = UserEntity::find().order_by_asc(UserColumn::CreatedAt);

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
