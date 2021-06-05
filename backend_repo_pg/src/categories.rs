use crate::errors::PgRepoError;
use crate::filters::GetAllCategoriesFilter;
use crate::models::{db_models, domain};
use crate::options::{CategorySortType, PaginationOptions};
use crate::schema::categories;
use crate::{change_sets::UpdateCategory, insertables::NewCategory};
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};

pub struct CategoryRepo<'a> {
    conn: &'a crate::pg_util::RepoConnection,
}

impl<'a> CategoryRepo<'a> {
    pub fn new(conn: &'a crate::pg_util::RepoConnection) -> Self {
        Self { conn }
    }

    pub fn insert_one(&self, new_category: &NewCategory) -> Result<i32, diesel::result::Error> {
        let conn = &self.conn.pg_conn;
        let query = diesel::insert_into(categories::table).values(new_category);
        let inserted_category: db_models::Category = match query.get_result(conn).optional()? {
            None => return Err(diesel::result::Error::__Nonexhaustive),
            Some(value) => value,
        };
        Ok(inserted_category.id)
    }

    pub fn update_one(
        &self,
        id_value: i32,
        updated_category: &UpdateCategory,
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::categories::dsl::{categories, id};
        let conn = &self.conn.pg_conn;
        let query = diesel::update(categories.filter(id.eq(id_value))).set(updated_category);
        Ok(query.execute(conn)?)
    }

    pub fn delete_one(&self, id_value: i32) -> Result<usize, diesel::result::Error> {
        use crate::schema::categories::dsl::{categories, id};
        let conn = &self.conn.pg_conn;
        let query = diesel::delete(categories.filter(id.eq(id_value)));
        Ok(query.execute(conn)?)
    }

    pub fn find_one(
        &self,
        id_value: i32,
    ) -> Result<Option<domain::Category>, diesel::result::Error> {
        use crate::schema::categories::dsl::{categories, id};

        let conn = &self.conn.pg_conn;
        let query = categories
            .filter(id.eq(id_value))
            .select(categories::all_columns());
        let category: db_models::Category = match query.first(conn).optional()? {
            Some(value) => value,
            None => return Ok(None),
        };

        Ok(Some(domain::Category::from(category)))
    }

    pub fn find(
        &self,
        filter: GetAllCategoriesFilter,
        sort: Option<CategorySortType>,
        pagination: PaginationOptions,
    ) -> Result<(Vec<domain::Category>, i64), diesel::result::Error> {
        use crate::schema::categories::dsl::categories;
        let q = categories
            .select((
                categories::all_columns(),
                diesel::dsl::sql::<diesel::sql_types::BigInt>("count(*) over()"),
            ))
            .into_boxed();

        let q = if let (Some(page), Some(page_size)) = (pagination.page, pagination.page_size) {
            q.offset((page - 1) * page_size).limit(page_size)
        } else {
            q
        };

        let conn = &self.conn.pg_conn;
        let results: Vec<(db_models::Category, i64)> = q.load(conn)?;

        let count = match results.get(0) {
            Some((_, value)) => *value,
            None => 0,
        };
        let categories_list = results
            .into_iter()
            .map(|(link, _)| domain::Category::from(link))
            .collect::<Vec<_>>();
        Ok((categories_list, count))
    }
}
