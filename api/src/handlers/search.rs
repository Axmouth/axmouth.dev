use crate::{errors::AppError, util::paginated_ok_response};
use axum::{extract::Extension, response::IntoResponse};
use backend_repo_pg::{
    errors::PgRepoError,
    filters::GetAllSearchItemsFilter,
    models::queries::{GetAllSearchItemsQuery, PaginatedQuery},
    pg_util::DynRepo,
    search_items::SearchItemRepo,
};
use tokio::task::block_in_place;

pub async fn get_all(
    query: GetAllSearchItemsQuery,
    Extension(repo): Extension<DynRepo>,
) -> Result<impl IntoResponse, AppError> {
    block_in_place(|| {
        let conn = repo.get_conn()?;
        let filter = GetAllSearchItemsFilter::from_query(query.clone());
        let search_item_repository = SearchItemRepo::new(&conn);
        let pagination_opts = query.pagination_options();
        let (search_items_list, total_results) = search_item_repository
            .find(filter, None, pagination_opts)
            .map_err::<PgRepoError, _>(|e| e.into())?;
        Ok(paginated_ok_response(
            search_items_list,
            query.page,
            query.page_size,
            total_results,
        ))
    })
}
