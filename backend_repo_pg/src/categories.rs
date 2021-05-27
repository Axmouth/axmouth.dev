use crate::filters::GetAllCategoriesFilter;
use crate::models::{db_models, domain};
use crate::options::{CategorySortType, PaginationOptions};
use crate::schema::categories;
use crate::{change_sets::UpdateCategory, insertables::NewCategory};
use crate::{errors::PgRepoError, pg_util::Repo};
use diesel::prelude::*;
use diesel::{r2d2::ConnectionManager, PgConnection, QueryDsl, RunQueryDsl};
use r2d2::Pool;

#[derive(Clone)]
pub struct CategoryRepo {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl CategoryRepo {
    pub fn new(repo: Repo) -> Self {
        Self { pool: repo.pool }
    }

    pub async fn insert_one(&self, new_category: &NewCategory) -> Result<i32, PgRepoError> {
        let conn = self.pool.get()?;
        let query = diesel::insert_into(categories::table).values(new_category);
        let inserted_category: db_models::Category =
            match tokio::task::block_in_place(move || query.get_result(&conn)).optional()? {
                None => {
                    return Err(PgRepoError {
                        error_message: "Failed to insert".to_string(),
                        error_type: crate::errors::PgRepoErrorType::Unknown,
                    })
                }
                Some(value) => value,
            };
        Ok(inserted_category.id)
    }

    pub async fn update_one(
        &self,
        id_value: i32,
        updated_category: &UpdateCategory,
    ) -> Result<usize, PgRepoError> {
        use crate::schema::categories::dsl::{categories, id};
        let conn = self.pool.get()?;
        let query = diesel::update(categories.filter(id.eq(id_value))).set(updated_category);
        Ok(tokio::task::block_in_place(move || query.execute(&conn))?)
    }

    pub async fn delete_one(&self, id_value: i32) -> Result<usize, PgRepoError> {
        use crate::schema::categories::dsl::{categories, id};
        let conn = self.pool.get()?;
        let query = diesel::delete(categories.filter(id.eq(id_value)));
        Ok(tokio::task::block_in_place(move || query.execute(&conn))?)
    }

    pub async fn find_one(&self, id_value: i32) -> Result<Option<domain::Category>, PgRepoError> {
        use crate::schema::categories::dsl::{categories, id};

        let conn = self.pool.get()?;
        let query = categories
            .filter(id.eq(id_value))
            .select(categories::all_columns());
        let category: db_models::Category =
            match tokio::task::block_in_place(move || query.first(&conn).optional())? {
                Some(value) => value,
                None => return Ok(None),
            };

        Ok(Some(domain::Category::from(category)))
    }

    pub async fn find(
        &self,
        filter: GetAllCategoriesFilter,
        sort: Option<CategorySortType>,
        pagination: PaginationOptions,
    ) -> Result<(Vec<domain::Category>, i64), PgRepoError> {
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

        let conn = self.pool.get()?;
        let results: Vec<(db_models::Category, i64)> =
            tokio::task::block_in_place(move || q.load(&conn))?;

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
