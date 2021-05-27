use crate::filters::GetAllTechnologiesFilter;
use crate::models::{db_models, domain};
use crate::options::{PaginationOptions, TechnologySortType};
use crate::schema::technologies;
use crate::{change_sets::UpdateTechnology, insertables::NewTechnology};
use crate::{errors::PgRepoError, pg_util::Repo};
use diesel::prelude::*;
use diesel::{r2d2::ConnectionManager, PgConnection, QueryDsl, RunQueryDsl};
use r2d2::Pool;

#[derive(Clone)]
pub struct TechnologyRepo {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl TechnologyRepo {
    pub fn new(repo: Repo) -> Self {
        Self { pool: repo.pool }
    }

    pub async fn insert_one(&self, new_technology: &NewTechnology) -> Result<i32, PgRepoError> {
        let conn = self.pool.get()?;
        let query = diesel::insert_into(technologies::table).values(new_technology);
        let inserted_technology: db_models::Technology =
            match tokio::task::block_in_place(move || query.get_result(&conn)).optional()? {
                None => {
                    return Err(PgRepoError {
                        error_message: "Failed to insert".to_string(),
                        error_type: crate::errors::PgRepoErrorType::Unknown,
                    })
                }
                Some(value) => value,
            };
        Ok(inserted_technology.id)
    }

    pub async fn update_one(
        &self,
        id_value: i32,
        updated_tech: &UpdateTechnology,
    ) -> Result<usize, PgRepoError> {
        use crate::schema::technologies::dsl::{id, technologies};
        let conn = self.pool.get()?;
        let query = diesel::update(technologies.filter(id.eq(id_value))).set(updated_tech);
        Ok(tokio::task::block_in_place(move || query.execute(&conn))?)
    }

    pub async fn delete_one(&self, id_value: i32) -> Result<usize, PgRepoError> {
        use crate::schema::technologies::dsl::{id, technologies};
        let conn = self.pool.get()?;
        let query = diesel::delete(technologies.filter(id.eq(id_value)));
        Ok(tokio::task::block_in_place(move || query.execute(&conn))?)
    }

    pub async fn find_one(&self, id_value: i32) -> Result<Option<domain::Technology>, PgRepoError> {
        use crate::schema::technologies::dsl::{id, technologies};

        let conn = self.pool.get()?;
        let query = technologies
            .filter(id.eq(id_value))
            .select(technologies::all_columns());
        let technology: db_models::Technology =
            match tokio::task::block_in_place(move || query.first(&conn).optional())? {
                Some(value) => value,
                None => return Ok(None),
            };

        Ok(Some(domain::Technology::from(technology)))
    }

    pub async fn find(
        &self,
        filter: GetAllTechnologiesFilter,
        sort: Option<TechnologySortType>,
        pagination: PaginationOptions,
    ) -> Result<(Vec<domain::Technology>, i64), PgRepoError> {
        use crate::schema::technologies::dsl::technologies;
        let q = technologies
            .select((
                technologies::all_columns(),
                diesel::dsl::sql::<diesel::sql_types::BigInt>("count(*) over()"),
            ))
            .into_boxed();

        let q = if let (Some(page), Some(page_size)) = (pagination.page, pagination.page_size) {
            q.offset((page - 1) * page_size).limit(page_size)
        } else {
            q
        };

        let conn = self.pool.get()?;
        let results: Vec<(db_models::Technology, i64)> =
            tokio::task::block_in_place(move || q.load(&conn))?;

        let count = match results.get(0) {
            Some((_, value)) => *value,
            None => 0,
        };
        let technologies_list = results
            .into_iter()
            .map(|(link, _)| domain::Technology::from(link))
            .collect::<Vec<_>>();
        Ok((technologies_list, count))
    }
}
