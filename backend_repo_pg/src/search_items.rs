use crate::filters::GetAllSearchItemsFilter;
use crate::models::{db_models, domain};
use crate::options::{PaginationOptions, SearchItemsSortType};
use diesel::{QueryDsl, RunQueryDsl};
use diesel_full_text_search::{plainto_tsquery, TsVectorExtensions};

pub struct SearchItemRepo<'a> {
    conn: &'a crate::pg_util::RepoConnection,
}

impl<'a> SearchItemRepo<'a> {
    pub fn new(conn: &'a crate::pg_util::RepoConnection) -> Self {
        Self { conn }
    }

    pub fn find(
        &self,
        filter: GetAllSearchItemsFilter,
        sort: Option<SearchItemsSortType>,
        pagination: PaginationOptions,
    ) -> Result<(Vec<domain::SearchItem>, i64), diesel::result::Error> {
        use crate::schema_extra::search_items::dsl::{
            description, id, item_type, link, search_items as search_items_dsl, search_vec, title,
        };
        let q = search_items_dsl
            .select((
                (id, title, description, item_type, link),
                diesel::dsl::sql::<diesel::sql_types::BigInt>("count(*) over()"),
            ))
            .into_boxed();

        let q = if let Some(search_text) = filter.search_text {
            q.filter(search_vec.matches(plainto_tsquery(search_text)))
        } else {
            q
        };

        let q = if let (Some(page), Some(page_size)) = (pagination.page, pagination.page_size) {
            q.offset((page - 1) * page_size).limit(page_size)
        } else {
            q
        };

        let conn = &self.conn.pg_conn;
        let results: Vec<(db_models::SearchItem, i64)> = q.load(conn)?;

        let count = match results.get(0) {
            Some((_, value)) => *value,
            None => 0,
        };
        let search_item_results = results
            .into_iter()
            .map(|(search_item, _)| domain::SearchItem::from(search_item))
            .collect::<Vec<_>>();
        Ok((search_item_results, count))
    }
}
