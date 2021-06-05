use crate::errors::PgRepoError;
use crate::filters::GetAllTechnologiesFilter;
use crate::models::{db_models, domain};
use crate::options::{PaginationOptions, TechnologySortType};
use crate::schema::technologies;
use crate::{change_sets::UpdateTechnology, insertables::NewTechnology};
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};

pub struct TechnologyRepo<'a> {
    conn: &'a crate::pg_util::RepoConnection,
}

impl<'a> TechnologyRepo<'a> {
    pub fn new(conn: &'a crate::pg_util::RepoConnection) -> Self {
        Self { conn }
    }

    pub fn insert_one(&self, new_technology: &NewTechnology) -> Result<i32, diesel::result::Error> {
        let conn = &self.conn.pg_conn;
        let query = diesel::insert_into(technologies::table).values(new_technology);
        let inserted_technology: db_models::Technology = match query.get_result(conn).optional()? {
            None => return Err(diesel::result::Error::__Nonexhaustive),
            Some(value) => value,
        };
        Ok(inserted_technology.id)
    }

    pub fn update_one(
        &self,
        id_value: i32,
        updated_tech: &UpdateTechnology,
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::technologies::dsl::{id, technologies};
        let conn = &self.conn.pg_conn;
        let query = diesel::update(technologies.filter(id.eq(id_value))).set(updated_tech);
        Ok(query.execute(conn)?)
    }

    pub fn delete_one(&self, id_value: i32) -> Result<usize, diesel::result::Error> {
        use crate::schema::technologies::dsl::{id, technologies};
        let conn = &self.conn.pg_conn;
        let query = diesel::delete(technologies.filter(id.eq(id_value)));
        Ok(query.execute(conn)?)
    }

    pub fn find_one(
        &self,
        id_value: i32,
    ) -> Result<Option<domain::Technology>, diesel::result::Error> {
        use crate::schema::technologies::dsl::{id, technologies};

        let conn = &self.conn.pg_conn;
        let query = technologies
            .filter(id.eq(id_value))
            .select(technologies::all_columns());
        let technology: db_models::Technology = match query.first(conn).optional()? {
            Some(value) => value,
            None => return Ok(None),
        };

        Ok(Some(domain::Technology::from(technology)))
    }

    pub fn find(
        &self,
        filter: GetAllTechnologiesFilter,
        sort: Option<TechnologySortType>,
        pagination: PaginationOptions,
    ) -> Result<(Vec<domain::Technology>, i64), diesel::result::Error> {
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

        let conn = &self.conn.pg_conn;
        let results: Vec<(db_models::Technology, i64)> = q.load(conn)?;

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
