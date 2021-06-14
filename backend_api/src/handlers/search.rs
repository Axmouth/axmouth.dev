use crate::{app::AppState, util::paginated_ok_response};
use backend_repo_pg::{
    filters::GetAllSearchItemsFilter, models::queries::GetAllSearchItemsQuery,
    options::PaginationOptions, search_items::SearchItemRepo,
};

pub async fn get_all(
    query: GetAllSearchItemsQuery,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(state
        .repo
        .transaction(|conn| {
            let filter = GetAllSearchItemsFilter::from_query(query.clone());
            let search_item_repository = SearchItemRepo::new(&conn);
            let (search_items_list, total_results) = search_item_repository.find(
                filter,
                None,
                PaginationOptions {
                    page: query.page,
                    page_size: query.page_size,
                },
            )?;
            Ok(paginated_ok_response(
                search_items_list,
                query.page,
                query.page_size,
                total_results,
            ))
        })
        .await?)
}
