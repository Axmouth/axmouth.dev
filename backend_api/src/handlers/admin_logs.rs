use crate::{
    app::AppState,
    auth_tokens::Claims,
    util::{not_found_response, paginated_ok_response, simple_ok_response},
};
use backend_repo_pg::{
    admin_logs::AdminLogRepo, filters::GetAllAdminLogsFilter,
    models::queries::GetAllAdminLogsQuery, options::PaginationOptions,
};

pub async fn get(id: i32, _: Claims, state: AppState) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(state
        .repo
        .transaction(|conn| {
            let admin_log_repository = AdminLogRepo::new(&conn);
            let admin_log = match admin_log_repository.find_one(id)? {
                None => {
                    return Ok(not_found_response("User"));
                }
                Some(value) => value,
            };
            Ok(simple_ok_response(admin_log))
        })
        .await?)
}

pub async fn get_all(
    _: Claims,
    query: GetAllAdminLogsQuery,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(state
        .repo
        .transaction(|conn| {
            let filter = GetAllAdminLogsFilter::from_query(query.clone());
            let admin_log_repository = AdminLogRepo::new(&conn);
            let (admin_logs, total_results) = admin_log_repository.find(
                filter,
                query.sort_type,
                PaginationOptions {
                    page: query.page,
                    page_size: query.page_size,
                },
            )?;
            Ok(paginated_ok_response(
                admin_logs,
                query.page,
                query.page_size,
                total_results,
            ))
        })
        .await?)
}
