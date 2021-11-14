use crate::errors::PgRepoError;
use crate::filters::GetAllPageViewsFilter;
use crate::insertables::NewPageView;
use crate::models::{db_models, domain};
use crate::options::{PageViewSortType, PaginationOptions};
use crate::schema::page_views;
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};

pub struct PageViewRepo<'a> {
    conn: &'a crate::pg_util::RepoConnection,
}

impl<'a> PageViewRepo<'a> {
    pub fn new(conn: &'a crate::pg_util::RepoConnection) -> Self {
        Self { conn }
    }

    pub fn insert_one(
        &self,
        new_text_body: NewPageView,
    ) -> Result<domain::PageView, diesel::result::Error> {
        let conn = &self.conn.pg_conn;
        let query = diesel::insert_into(page_views::table).values(&new_text_body);
        let result = query.get_result(conn)?;
        Ok(domain::PageView::from(result))
    }

    pub fn delete_one(&self, id_value: i32) -> Result<usize, diesel::result::Error> {
        use crate::schema::page_views::dsl::{id, page_views};
        let conn = &self.conn.pg_conn;
        let query = diesel::delete(page_views.filter(id.eq(id_value)));
        Ok(query.execute(conn)?)
    }

    pub fn find_one(
        &self,
        id_value: i32,
    ) -> Result<Option<domain::PageView>, diesel::result::Error> {
        use crate::schema::page_views::dsl::{id, page_views};

        let conn = &self.conn.pg_conn;
        let query = page_views
            .filter(id.eq(id_value))
            .select(page_views::all_columns());
        let text_body: db_models::PageView = match query.first(conn).optional()? {
            Some(value) => value,
            None => return Ok(None),
        };
        Ok(Some(domain::PageView::from(text_body)))
    }

    pub fn find_one_by_url(
        &self,
        url_value: String,
    ) -> Result<Option<domain::PageView>, diesel::result::Error> {
        use crate::schema::page_views::dsl::{page_url, page_views};

        let conn = &self.conn.pg_conn;
        let query = page_views
            .filter(page_url.eq(url_value))
            .select(page_views::all_columns());
        let text_body: db_models::PageView = match query.first(conn).optional()? {
            Some(value) => value,
            None => return Ok(None),
        };
        Ok(Some(domain::PageView::from(text_body)))
    }

    pub fn count_by_url(&self, url_value: &str) -> Result<i64, diesel::result::Error> {
        use crate::schema::page_views::dsl::{id_hash, page_url, page_views};

        let conn = &self.conn.pg_conn;
        let query = page_views
            .filter(page_url.eq(url_value))
            .select(diesel::dsl::sql::<diesel::sql_types::BigInt>(
                "Count(DISTINCT page_views.id_hash) ",
            ));

        let count: i64 = query.first(conn)?;

        Ok(count)
    }

    pub fn count_by_root_url(&self, url_value: String) -> Result<i64, diesel::result::Error> {
        use crate::schema::page_views::dsl::{id_hash, page_url, page_views};

        let conn = &self.conn.pg_conn;
        let query = page_views
            .filter(page_url.like(format!("{}%", url_value)))
            .select(diesel::dsl::sql::<diesel::sql_types::BigInt>(
                "Count(DISTINCT page_views.id_hash) ",
            ));
        let count: i64 = query.first(conn)?;

        Ok(count)
    }

    pub fn find(
        &self,
        filter: GetAllPageViewsFilter,
        sort: Option<PageViewSortType>,
        pagination: PaginationOptions,
    ) -> Result<Vec<domain::PageView>, diesel::result::Error> {
        use crate::schema::page_views::dsl::page_views;
        let q = page_views.select(page_views::all_columns()).into_boxed();

        let q = if let (Some(page), Some(page_size)) = (pagination.page, pagination.page_size) {
            q.offset((page - 1) * page_size).limit(page_size)
        } else {
            q
        };

        let conn = &self.conn.pg_conn;
        let results: Vec<db_models::PageView> = q.load(conn)?;

        Ok(results
            .into_iter()
            .map(|text_body| domain::PageView::from(text_body))
            .collect::<Vec<_>>())
    }
}
