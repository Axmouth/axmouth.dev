use crate::errors::PgRepoError;
use crate::filters::GetAllTextBodiesFilter;
use crate::models::{db_models, domain};
use crate::options::{PaginationOptions, TextBodySort};
use crate::schema::text_bodies;
use crate::{change_sets::UpdateTextBody, insertables::NewTextBody};
use diesel::prelude::*;
use diesel::{r2d2::ConnectionManager, PgConnection, QueryDsl, RunQueryDsl};
use r2d2::Pool;

#[derive(Clone)]
pub struct TextBodyRepo {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl TextBodyRepo {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }

    pub async fn insert_one(
        &self,
        new_text_body: NewTextBody,
    ) -> Result<domain::TextBody, PgRepoError> {
        let conn = self.pool.get()?;
        let query = diesel::insert_into(text_bodies::table).values(&new_text_body);
        let result = tokio::task::block_in_place(move || query.get_result(&conn))?;
        Ok(domain::TextBody::from(result))
    }

    pub async fn update_one(
        &self,
        id_value: i32,
        updated_text_body: UpdateTextBody,
    ) -> Result<domain::TextBody, PgRepoError> {
        use crate::schema::text_bodies::dsl::{id, text_bodies};
        let conn = self.pool.get()?;
        let query = diesel::update(text_bodies.filter(id.eq(id_value))).set(&updated_text_body);
        let result = tokio::task::block_in_place(move || query.get_result(&conn))?;
        Ok(domain::TextBody::from(result))
    }

    pub async fn delete_one(&self, id_value: i32) -> Result<usize, PgRepoError> {
        use crate::schema::text_bodies::dsl::{id, text_bodies};
        let conn = self.pool.get()?;
        let query = diesel::delete(text_bodies.filter(id.eq(id_value)));
        Ok(query.execute(&conn)?)
    }

    pub async fn find_one(&self, id_value: i32) -> Result<Option<domain::TextBody>, PgRepoError> {
        use crate::schema::text_bodies::dsl::{id, text_bodies};

        let conn = self.pool.get()?;
        let query = text_bodies
            .filter(id.eq(id_value))
            .select(text_bodies::all_columns());
        let text_body: db_models::TextBody =
            match tokio::task::block_in_place(move || query.first(&conn).optional())? {
                Some(value) => value,
                None => return Ok(None),
            };
        Ok(Some(domain::TextBody::from(text_body)))
    }

    pub async fn find_one_by_slug(
        &self,
        slug_value: String,
    ) -> Result<Option<domain::TextBody>, PgRepoError> {
        use crate::schema::text_bodies::dsl::{id, slug, text_bodies};

        let conn = self.pool.get()?;
        let query = text_bodies
            .filter(slug.eq(slug_value))
            .select(text_bodies::all_columns());
        let text_body: db_models::TextBody =
            match tokio::task::block_in_place(move || query.first(&conn).optional())? {
                Some(value) => value,
                None => return Ok(None),
            };
        Ok(Some(domain::TextBody::from(text_body)))
    }

    pub async fn find(
        &self,
        filter: GetAllTextBodiesFilter,
        sort: TextBodySort,
        pagination: PaginationOptions,
    ) -> Result<(Vec<domain::TextBody>, i64), PgRepoError> {
        use crate::schema::text_bodies::dsl::{id as text_body_id, text_bodies};
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

        let conn = self.pool.get()?;
        let results: Vec<(db_models::TextBody, i64)> =
            tokio::task::block_in_place(move || q.load(&conn))?;

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
