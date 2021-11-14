use crate::filters::GetAllUsersFilter;
use crate::models::{db_models, domain};
use crate::options::{PaginationOptions, UserSortType};
use crate::schema::users;
use crate::{change_sets::UpdateUser, insertables::NewUser};
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};

pub struct UserRepo<'a> {
    conn: &'a crate::pg_util::RepoConnection,
}

impl<'a> UserRepo<'a> {
    pub fn new(conn: &'a crate::pg_util::RepoConnection) -> Self {
        Self { conn }
    }

    pub fn insert_one(&self, new_user: NewUser) -> Result<domain::User, diesel::result::Error> {
        let conn = &self.conn.pg_conn;
        let query = diesel::insert_into(users::table).values(&new_user);
        let result = query.get_result(conn)?;
        Ok(domain::User::from(result))
    }

    pub fn update_one(
        &self,
        id_value: i32,
        updated_user: UpdateUser,
    ) -> Result<domain::User, diesel::result::Error> {
        use crate::schema::users::dsl::{id, users};
        let conn = &self.conn.pg_conn;
        let query = diesel::update(users.filter(id.eq(id_value))).set(&updated_user);
        let result = query.get_result(conn)?;
        Ok(domain::User::from(result))
    }

    pub fn delete_one(&self, id_value: i32) -> Result<usize, diesel::result::Error> {
        use crate::schema::users::dsl::{id, users};
        let conn = &self.conn.pg_conn;
        let query = diesel::delete(users.filter(id.eq(id_value)));
        Ok(query.execute(conn)?)
    }

    pub fn find_one(&self, id_value: i32) -> Result<Option<domain::User>, diesel::result::Error> {
        use crate::schema::users::dsl::{id, users};

        let conn = &self.conn.pg_conn;
        let query = users.filter(id.eq(id_value)).select(users::all_columns());
        let user: db_models::User = match query.first(conn).optional()? {
            Some(value) => value,
            None => return Ok(None),
        };
        Ok(Some(domain::User::from(user)))
    }

    pub fn find_one_by_email(
        &self,
        email_value: String,
    ) -> Result<Option<db_models::User>, diesel::result::Error> {
        use crate::schema::users::dsl::{email, users};

        let conn = &self.conn.pg_conn;
        let query = users
            .filter(email.eq(email_value))
            .select(users::all_columns());
        let user: db_models::User = match query.first(conn).optional()? {
            Some(value) => value,
            None => return Ok(None),
        };
        Ok(Some(user))
    }

    pub fn find_one_by_display_name(
        &self,
        display_name_value: String,
    ) -> Result<Option<db_models::User>, diesel::result::Error> {
        use crate::schema::users::dsl::{display_name, users};

        let conn = &self.conn.pg_conn;
        let query = users
            .filter(display_name.eq(display_name_value))
            .select(users::all_columns());
        let user: db_models::User = match query.first(conn).optional()? {
            Some(value) => value,
            None => return Ok(None),
        };
        Ok(Some(user))
    }

    pub fn find(
        &self,
        filter: GetAllUsersFilter,
        sort: Option<UserSortType>,
        pagination: PaginationOptions,
    ) -> Result<Vec<domain::User>, diesel::result::Error> {
        use crate::schema::users::dsl::users;
        let q = users.select(users::all_columns()).into_boxed();

        let q = if let (Some(page), Some(page_size)) = (pagination.page, pagination.page_size) {
            q.offset((page - 1) * page_size).limit(page_size)
        } else {
            q
        };

        let conn = &self.conn.pg_conn;
        let results: Vec<db_models::User> = q.load(conn)?;

        Ok(results
            .into_iter()
            .map(|user| domain::User::from(user))
            .collect::<Vec<_>>())
    }
}
