use crate::{app::AppState, util::paginated_ok_response};
use backend_repo_pg::{
    errors::PgRepoError, filters::GetAllSearchItemsFilter, models::queries::GetAllSearchItemsQuery,
    options::PaginationOptions, pg_util::RepoConnection, search_items::SearchItemRepo,
};
use tokio::task::block_in_place;

pub async fn get_all(
    query: GetAllSearchItemsQuery,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    block_in_place(|| {
        let conn = RepoConnection::new(state.repo)?;
        let filter = GetAllSearchItemsFilter::from_query(query.clone());
        let search_item_repository = SearchItemRepo::new(&conn);
        let (search_items_list, total_results) = search_item_repository
            .find(
                filter,
                None,
                PaginationOptions {
                    page: query.page,
                    page_size: query.page_size,
                },
            )
            .map_err::<PgRepoError, _>(|e| e.into())?;
        Ok(paginated_ok_response(
            search_items_list,
            query.page,
            query.page_size,
            total_results,
        ))
    })
}
