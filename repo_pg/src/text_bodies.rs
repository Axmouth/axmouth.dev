use crate::errors::PgRepoError;
use crate::filters::GetAllTextBodiesFilter;
use crate::models::{db_models, domain};
use crate::options::{PaginationOptions, TextBodySortType};
use crate::schema::text_bodies;
use crate::{change_sets::UpdateTextBody, insertables::NewTextBody};
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};

pub struct TextBodyRepo<'a> {
    conn: &'a crate::pg_util::RepoConnection,
}

impl<'a> TextBodyRepo<'a> {
    pub fn new(conn: &'a crate::pg_util::RepoConnection) -> Self {
        Self { conn }
    }

    pub fn insert_one(
        &self,
        new_text_body: &NewTextBody,
    ) -> Result<domain::TextBody, diesel::result::Error> {
        let conn = &self.conn.pg_conn;
        let query = diesel::insert_into(text_bodies::table).values(new_text_body);
        let result = query.get_result(conn)?;
        Ok(domain::TextBody::from(result))
    }

    pub fn update_one(
        &self,
        id_value: i32,
        updated_text_body: &UpdateTextBody,
    ) -> Result<domain::TextBody, diesel::result::Error> {
        use crate::schema::text_bodies::dsl::{id, text_bodies};
        let conn = &self.conn.pg_conn;
        let query = diesel::update(text_bodies.filter(id.eq(id_value))).set(updated_text_body);
        let result = query.get_result(conn)?;
        Ok(domain::TextBody::from(result))
    }

    pub fn delete_one(&self, id_value: i32) -> Result<usize, diesel::result::Error> {
        use crate::schema::text_bodies::dsl::{id, text_bodies};
        let conn = &self.conn.pg_conn;
        let query = diesel::delete(text_bodies.filter(id.eq(id_value)));
        Ok(query.execute(conn)?)
    }

    pub fn find_one(
        &self,
        id_value: i32,
    ) -> Result<Option<domain::TextBody>, diesel::result::Error> {
        use crate::schema::text_bodies::dsl::{id, text_bodies};

        let conn = &self.conn.pg_conn;
        let query = text_bodies
            .filter(id.eq(id_value))
            .select(text_bodies::all_columns());
        let text_body: db_models::TextBody = match query.first(conn).optional()? {
            Some(value) => value,
            None => return Ok(None),
        };
        Ok(Some(domain::TextBody::from(text_body)))
    }

    pub fn find_one_by_slug(
        &self,
        slug_value: String,
    ) -> Result<Option<domain::TextBody>, diesel::result::Error> {
        use crate::schema::text_bodies::dsl::{id, slug, text_bodies};

        let conn = &self.conn.pg_conn;
        let query = text_bodies
            .filter(slug.eq(slug_value))
            .select(text_bodies::all_columns());
        let text_body: db_models::TextBody = match query.first(conn).optional()? {
            Some(value) => value,
            None => return Ok(None),
        };
        Ok(Some(domain::TextBody::from(text_body)))
    }

    pub fn find(
        &self,
        filter: GetAllTextBodiesFilter,
        sort: Option<TextBodySortType>,
        pagination: PaginationOptions,
    ) -> Result<(Vec<domain::TextBody>, i64), diesel::result::Error> {
        use crate::schema::text_bodies::dsl::text_bodies;
        let q = text_bodies
            .select((
                text_bodies::all_columns(),
                diesel::dsl::sql::<diesel::sql_types::BigInt>("Count(*) Over()"),
            ))
            .into_boxed();

        let q = if let (Some(page), Some(page_size)) = (pagination.page, pagination.page_size) {
            q.offset((page - 1) * page_size).limit(page_size)
        } else {
            q
        };

        let conn = &self.conn.pg_conn;
        let results: Vec<(db_models::TextBody, i64)> = q.load(conn)?;

        let count = match results.get(0) {
            Some((_, value)) => *value,
            None => 0,
        };
        let text_bodies_list = results
            .into_iter()
            .map(|(text_body, _)| domain::TextBody::from(text_body))
            .collect::<Vec<_>>();
        Ok((text_bodies_list, count))
    }
}
