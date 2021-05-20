use crate::filters::GetAllHomePageLinksFilter;
use crate::models::{db_models, domain};
use crate::options::{HomePageLinkSortType, PaginationOptions};
use crate::schema::home_page_links;
use crate::{change_sets::UpdateHomePageLink, insertables::NewHomePageLink};
use crate::{errors::PgRepoError, pg_util::Repo};
use diesel::prelude::*;
use diesel::{r2d2::ConnectionManager, PgConnection, QueryDsl, RunQueryDsl};
use r2d2::Pool;

#[derive(Clone)]
pub struct HomePageLinkRepo {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl HomePageLinkRepo {
    pub fn new(repo: Repo) -> Self {
        Self { pool: repo.pool }
    }

    pub async fn insert_one(&self, new_comment: NewHomePageLink) -> Result<usize, PgRepoError> {
        let conn = self.pool.get()?;
        let query = diesel::insert_into(home_page_links::table).values(&new_comment);
        Ok(tokio::task::block_in_place(move || query.execute(&conn))?)
    }

    pub async fn update_one(
        &self,
        id_value: i32,
        updated_comment: UpdateHomePageLink,
    ) -> Result<usize, PgRepoError> {
        use crate::schema::home_page_links::dsl::{home_page_links, id};
        let conn = self.pool.get()?;
        let query = diesel::update(home_page_links.filter(id.eq(id_value))).set(&updated_comment);
        Ok(tokio::task::block_in_place(move || query.execute(&conn))?)
    }

    pub async fn delete_one(&self, id_value: i32) -> Result<usize, PgRepoError> {
        use crate::schema::home_page_links::dsl::{home_page_links, id};
        let conn = self.pool.get()?;
        let query = diesel::delete(home_page_links.filter(id.eq(id_value)));
        Ok(tokio::task::block_in_place(move || query.execute(&conn))?)
    }

    pub async fn find_one(
        &self,
        id_value: i32,
    ) -> Result<Option<domain::HomePageLink>, PgRepoError> {
        use crate::schema::home_page_links::dsl::{home_page_links, id};

        let conn = self.pool.get()?;
        let query = home_page_links
            .filter(id.eq(id_value))
            .select(home_page_links::all_columns());
        let home_page_link: db_models::HomePageLink =
            match tokio::task::block_in_place(move || query.first(&conn).optional())? {
                Some(value) => value,
                None => return Ok(None),
            };

        Ok(Some(domain::HomePageLink::from(home_page_link)))
    }

    pub async fn find(
        &self,
        filter: GetAllHomePageLinksFilter,
        sort: Option<HomePageLinkSortType>,
        pagination: PaginationOptions,
    ) -> Result<(Vec<domain::HomePageLink>, i64), PgRepoError> {
        use crate::schema::home_page_links::dsl::home_page_links;
        let q = home_page_links
            .select((
                home_page_links::all_columns(),
                diesel::dsl::sql::<diesel::sql_types::BigInt>("count(*) over()"),
            ))
            .into_boxed();

        let q = if let (Some(page), Some(page_size)) = (pagination.page, pagination.page_size) {
            q.offset((page - 1) * page_size).limit(page_size)
        } else {
            q
        };

        let conn = self.pool.get()?;
        let results: Vec<(db_models::HomePageLink, i64)> =
            tokio::task::block_in_place(move || q.load(&conn))?;

        let count = match results.get(0) {
            Some((_, value)) => *value,
            None => 0,
        };
        let home_page_links_list = results
            .into_iter()
            .map(|(link, _)| domain::HomePageLink::from(link))
            .collect::<Vec<_>>();
        Ok((home_page_links_list, count))
    }
}
