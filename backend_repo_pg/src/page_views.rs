use crate::filters::GetAllPageViewsFilter;
use crate::insertables::NewPageView;
use crate::models::{db_models, domain};
use crate::options::{PageViewSortType, PaginationOptions};
use crate::schema::page_views;
use crate::{errors::PgRepoError, pg_util::Repo};
use diesel::{dsl::count_star, prelude::*};
use diesel::{r2d2::ConnectionManager, PgConnection, QueryDsl, RunQueryDsl};
use r2d2::Pool;

#[derive(Clone)]
pub struct PageViewRepo {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl PageViewRepo {
    pub fn new(repo: Repo) -> Self {
        Self { pool: repo.pool }
    }

    pub async fn insert_one(
        &self,
        new_text_body: NewPageView,
    ) -> Result<domain::PageView, PgRepoError> {
        let conn = self.pool.get()?;
        let query = diesel::insert_into(page_views::table).values(&new_text_body);
        let result = tokio::task::block_in_place(move || query.get_result(&conn))?;
        Ok(domain::PageView::from(result))
    }

    pub async fn delete_one(&self, id_value: i32) -> Result<usize, PgRepoError> {
        use crate::schema::page_views::dsl::{id, page_views};
        let conn = self.pool.get()?;
        let query = diesel::delete(page_views.filter(id.eq(id_value)));
        Ok(query.execute(&conn)?)
    }

    pub async fn find_one(&self, id_value: i32) -> Result<Option<domain::PageView>, PgRepoError> {
        use crate::schema::page_views::dsl::{id, page_views};

        let conn = self.pool.get()?;
        let query = page_views
            .filter(id.eq(id_value))
            .select(page_views::all_columns());
        let text_body: db_models::PageView =
            match tokio::task::block_in_place(move || query.first(&conn).optional())? {
                Some(value) => value,
                None => return Ok(None),
            };
        Ok(Some(domain::PageView::from(text_body)))
    }

    pub async fn find_one_by_url(
        &self,
        url_value: String,
    ) -> Result<Option<domain::PageView>, PgRepoError> {
        use crate::schema::page_views::dsl::{page_url, page_views};

        let conn = self.pool.get()?;
        let query = page_views
            .filter(page_url.eq(url_value))
            .select(page_views::all_columns());
        let text_body: db_models::PageView =
            match tokio::task::block_in_place(move || query.first(&conn).optional())? {
                Some(value) => value,
                None => return Ok(None),
            };
        Ok(Some(domain::PageView::from(text_body)))
    }

    pub async fn count_by_url(&self, url_value: String) -> Result<i64, PgRepoError> {
        use crate::schema::page_views::dsl::{id_hash, page_url, page_views};

        let conn = self.pool.get()?;
        let query = page_views
            .filter(page_url.eq(url_value))
            .select(count_star())
            .distinct_on(id_hash);
        let count: i64 = tokio::task::block_in_place(move || query.first(&conn))?;

        Ok(count)
    }

    pub async fn count_by_root_url(&self, url_value: String) -> Result<i64, PgRepoError> {
        use crate::schema::page_views::dsl::{id_hash, page_url, page_views};

        let conn = self.pool.get()?;
        let query = page_views
            .filter(page_url.like(format!("{}%", url_value)))
            .select(count_star())
            .distinct_on(id_hash);
        let count: i64 = tokio::task::block_in_place(move || query.first(&conn))?;

        Ok(count)
    }

    pub async fn find(
        &self,
        filter: GetAllPageViewsFilter,
        sort: Option<PageViewSortType>,
        pagination: PaginationOptions,
    ) -> Result<Vec<domain::PageView>, PgRepoError> {
        use crate::schema::page_views::dsl::page_views;
        let q = page_views.select(page_views::all_columns()).into_boxed();

        let q = if let (Some(page), Some(page_size)) = (pagination.page, pagination.page_size) {
            q.offset((page - 1) * page_size).limit(page_size)
        } else {
            q
        };

        let conn = self.pool.get()?;
        let results: Vec<db_models::PageView> = tokio::task::block_in_place(move || q.load(&conn))?;

        Ok(results
            .into_iter()
            .map(|text_body| domain::PageView::from(text_body))
            .collect::<Vec<_>>())
    }
}
