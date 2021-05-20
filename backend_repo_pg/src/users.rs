use crate::filters::GetAllUsersFilter;
use crate::models::{db_models, domain};
use crate::options::{PaginationOptions, UserSortType};
use crate::schema::users;
use crate::{change_sets::UpdateUser, insertables::NewUser};
use crate::{errors::PgRepoError, pg_util::Repo};
use diesel::prelude::*;
use diesel::{r2d2::ConnectionManager, PgConnection, QueryDsl, RunQueryDsl};
use r2d2::Pool;

#[derive(Clone)]
pub struct UserRepo {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl UserRepo {
    pub fn new(repo: Repo) -> Self {
        Self { pool: repo.pool }
    }

    pub async fn insert_one(&self, new_user: NewUser) -> Result<domain::User, PgRepoError> {
        let conn = self.pool.get()?;
        let query = diesel::insert_into(users::table).values(&new_user);
        let result = tokio::task::block_in_place(move || query.get_result(&conn))?;
        Ok(domain::User::from(result))
    }

    pub async fn update_one(
        &self,
        id_value: i32,
        updated_user: UpdateUser,
    ) -> Result<domain::User, PgRepoError> {
        use crate::schema::users::dsl::{id, users};
        let conn = self.pool.get()?;
        let query = diesel::update(users.filter(id.eq(id_value))).set(&updated_user);
        let result = tokio::task::block_in_place(move || query.get_result(&conn))?;
        Ok(domain::User::from(result))
    }

    pub async fn delete_one(&self, id_value: i32) -> Result<usize, PgRepoError> {
        use crate::schema::users::dsl::{id, users};
        let conn = self.pool.get()?;
        let query = diesel::delete(users.filter(id.eq(id_value)));
        Ok(tokio::task::block_in_place(move || query.execute(&conn))?)
    }

    pub async fn find_one(&self, id_value: i32) -> Result<Option<domain::User>, PgRepoError> {
        use crate::schema::users::dsl::{id, users};

        let conn = self.pool.get()?;
        let query = users.filter(id.eq(id_value)).select(users::all_columns());
        let user: db_models::User =
            match tokio::task::block_in_place(move || query.first(&conn).optional())? {
                Some(value) => value,
                None => return Ok(None),
            };
        Ok(Some(domain::User::from(user)))
    }

    pub async fn find_one_by_email(
        &self,
        email_value: String,
    ) -> Result<Option<db_models::User>, PgRepoError> {
        use crate::schema::users::dsl::{email, users};

        let conn = self.pool.get()?;
        let query = users
            .filter(email.eq(email_value))
            .select(users::all_columns());
        let user: db_models::User =
            match tokio::task::block_in_place(move || query.first(&conn).optional())? {
                Some(value) => value,
                None => return Ok(None),
            };
        Ok(Some(user))
    }

    pub async fn find_one_by_display_name(
        &self,
        display_name_value: String,
    ) -> Result<Option<db_models::User>, PgRepoError> {
        use crate::schema::users::dsl::{display_name, users};

        let conn = self.pool.get()?;
        let query = users
            .filter(display_name.eq(display_name_value))
            .select(users::all_columns());
        let user: db_models::User =
            match tokio::task::block_in_place(move || query.first(&conn).optional())? {
                Some(value) => value,
                None => return Ok(None),
            };
        Ok(Some(user))
    }

    pub async fn find(
        &self,
        filter: GetAllUsersFilter,
        sort: Option<UserSortType>,
        pagination: PaginationOptions,
    ) -> Result<Vec<domain::User>, PgRepoError> {
        use crate::schema::users::dsl::users;
        let q = users.select(users::all_columns()).into_boxed();

        let q = if let (Some(page), Some(page_size)) = (pagination.page, pagination.page_size) {
            q.offset((page - 1) * page_size).limit(page_size)
        } else {
            q
        };

        let conn = self.pool.get()?;
        let results: Vec<db_models::User> = tokio::task::block_in_place(move || q.load(&conn))?;

        Ok(results
            .into_iter()
            .map(|user| domain::User::from(user))
            .collect::<Vec<_>>())
    }
}
