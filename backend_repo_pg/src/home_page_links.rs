use crate::errors::PgRepoError;
use crate::filters::GetAllHomePageLinksFilter;
use crate::models::{db_models, domain};
use crate::options::{HomePageLinkSortType, PaginationOptions};
use crate::schema::home_page_links;
use crate::{change_sets::UpdateHomePageLink, insertables::NewHomePageLink};
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};

pub struct HomePageLinkRepo<'a> {
    conn: &'a crate::pg_util::RepoConnection,
}

impl<'a> HomePageLinkRepo<'a> {
    pub fn new(conn: &'a crate::pg_util::RepoConnection) -> Self {
        Self { conn }
    }

    pub fn insert_one(
        &self,
        new_home_page_link: NewHomePageLink,
    ) -> Result<i32, diesel::result::Error> {
        let conn = &self.conn.pg_conn;
        let query = diesel::insert_into(home_page_links::table).values(&new_home_page_link);
        let inserted_link: db_models::HomePageLink = match query.get_result(conn).optional()? {
            None => return Err(diesel::result::Error::__Nonexhaustive),
            Some(value) => value,
        };
        Ok(inserted_link.id)
    }

    pub fn update_one(
        &self,
        id_value: i32,
        updated_link: &UpdateHomePageLink,
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::home_page_links::dsl::{home_page_links, id};
        let conn = &self.conn.pg_conn;
        let query = diesel::update(home_page_links.filter(id.eq(id_value))).set(updated_link);
        Ok(query.execute(conn)?)
    }

    pub fn delete_one(&self, id_value: i32) -> Result<usize, diesel::result::Error> {
        use crate::schema::home_page_links::dsl::{home_page_links, id};
        let conn = &self.conn.pg_conn;
        let query = diesel::delete(home_page_links.filter(id.eq(id_value)));
        Ok(query.execute(conn)?)
    }

    pub fn find_one(
        &self,
        id_value: i32,
    ) -> Result<Option<domain::HomePageLink>, diesel::result::Error> {
        use crate::schema::home_page_links::dsl::{home_page_links, id};

        let conn = &self.conn.pg_conn;
        let query = home_page_links
            .filter(id.eq(id_value))
            .select(home_page_links::all_columns());
        let home_page_link: db_models::HomePageLink = match query.first(conn).optional()? {
            Some(value) => value,
            None => return Ok(None),
        };

        Ok(Some(domain::HomePageLink::from(home_page_link)))
    }

    pub fn find(
        &self,
        filter: GetAllHomePageLinksFilter,
        sort: Option<HomePageLinkSortType>,
        pagination: PaginationOptions,
    ) -> Result<(Vec<domain::HomePageLink>, i64), diesel::result::Error> {
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

        let conn = &self.conn.pg_conn;
        let results: Vec<(db_models::HomePageLink, i64)> = q.load(conn)?;

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
