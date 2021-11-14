use crate::{
    errors::AppError,
    extractors::AdminClaimsContext,
    util::{paginated_ok_response, simple_ok_response},
};
use axum::{
    extract::{Extension, Path},
    response::IntoResponse,
};
use backend_repo_pg::{
    admin_logs::AdminLogRepo,
    errors::PgRepoError,
    filters::GetAllAdminLogsFilter,
    models::queries::{GetAllAdminLogsQuery, PaginatedQuery},
    pg_util::{pg_transaction, DynRepo},
};
use tokio::task::block_in_place;

pub async fn get(
    Path(id): Path<i32>,
    _: AdminClaimsContext,
    Extension(repo): Extension<DynRepo>,
) -> Result<impl IntoResponse, AppError> {
    let admin_log = pg_transaction(repo, |conn| {
        let admin_log_repository = AdminLogRepo::new(conn);
        admin_log_repository.find_one(id)
    })
    .await?
    .ok_or(AppError::NotFound("User"))?;
    Ok(simple_ok_response(admin_log))
}

pub async fn get_all(
    _: AdminClaimsContext,
    query: GetAllAdminLogsQuery,
    Extension(repo): Extension<DynRepo>,
) -> Result<impl IntoResponse, AppError> {
    block_in_place(|| {
        let conn = repo.get_conn()?;
        let filter = GetAllAdminLogsFilter::from_query(query.clone());
        let admin_log_repository = AdminLogRepo::new(&conn);
        let pagination_opts = query.pagination_options();
        let sort_type = query.sort_type;
        let (admin_logs, total_results) = admin_log_repository
            .find(filter, sort_type, pagination_opts)
            .map_err::<PgRepoError, _>(|e| e.into())?;
        Ok(paginated_ok_response(
            admin_logs,
            query.page,
            query.page_size,
            total_results,
        ))
    })
}
