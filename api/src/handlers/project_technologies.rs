use crate::errors::AppError;
use crate::extractors::AdminClaimsContext;
use crate::extractors::ValidatedJson;
use crate::util::create_creation_admin_log;
use crate::util::create_deletion_admin_log;
use crate::util::create_update_admin_log;
use crate::util::{
    not_found_response, paginated_ok_response, server_error_response, simple_created_response,
    simple_no_content_response, simple_ok_response,
};
use axum::extract::Extension;
use axum::extract::Path;
use axum::response::IntoResponse;
use backend_repo_pg::errors::PgRepoError;
use backend_repo_pg::models::queries::PaginatedQuery;
use backend_repo_pg::pg_util::pg_transaction;
use backend_repo_pg::pg_util::DynRepo;
use backend_repo_pg::technologies::TechnologyRepo;
use backend_repo_pg::{
    change_sets::UpdateTechnology,
    filters::GetAllTechnologiesFilter,
    insertables::NewTechnology,
    models::{
        queries::GetAllTechnologiesQuery,
        requests::{CreateTechnologyRequest, UpdateTechnologyRequest},
    },
};
use tokio::task::block_in_place;

pub async fn get(
    Path(id): Path<i32>,
    Extension(repo): Extension<DynRepo>,
) -> Result<impl IntoResponse, AppError> {
    block_in_place(|| {
        let conn = repo.get_conn()?;
        let technology_repository = TechnologyRepo::new(&conn);
        let technology_result = match technology_repository
            .find_one(id)
            .map_err::<PgRepoError, _>(|e| e.into())?
        {
            None => {
                return Ok(not_found_response("Technology"));
            }
            Some(value) => value,
        };
        Ok(simple_ok_response(technology_result))
    })
}

pub async fn get_all(
    query: GetAllTechnologiesQuery,
    Extension(repo): Extension<DynRepo>,
) -> Result<impl IntoResponse, AppError> {
    block_in_place(|| {
        let conn = repo.get_conn()?;
        let filter = GetAllTechnologiesFilter::from_query(query.clone());
        let technology_repository = TechnologyRepo::new(&conn);
        let pagination_opts = query.pagination_options();
        let sort_type = query.sort_type;
        let (technologies_list, total_results) = technology_repository
            .find(filter, sort_type, pagination_opts)
            .map_err::<PgRepoError, _>(|e| e.into())?;
        Ok(paginated_ok_response(
            technologies_list,
            query.page,
            query.page_size,
            total_results,
        ))
    })
}

pub async fn delete(
    Path(id): Path<i32>,
    AdminClaimsContext { claims }: AdminClaimsContext,
    Extension(repo): Extension<DynRepo>,
) -> Result<impl IntoResponse, AppError> {
    Ok(pg_transaction(repo, |conn| {
        let technology_repository = TechnologyRepo::new(conn);
        let old_data = match technology_repository.find_one(id) {
            Err(err) => {
                return Ok(server_error_response(err));
            }
            Ok(value_opt) => match value_opt {
                None => {
                    return Ok(not_found_response("Technology"));
                }
                Some(value) => value,
            },
        };
        let technology_result = match technology_repository.delete_one(id) {
            Err(err) => {
                return Ok(server_error_response(err));
            }
            Ok(value) => value,
        };
        if technology_result == 0 {
            return Ok(not_found_response("Technology"));
        }
        match create_deletion_admin_log(
            id.to_string(),
            claims.user_id(),
            String::from("Project Technology"),
            String::from("technologies"),
            &old_data,
            String::from("/technologies"),
            conn,
        ) {
            Ok(_) => {}
            Err(err) => {
                return Ok(server_error_response(err));
            }
        };
        Ok(simple_no_content_response(technology_result))
    })
    .await?)
}

pub async fn update(
    Path(id): Path<i32>,
    AdminClaimsContext { claims }: AdminClaimsContext,
    ValidatedJson(request): ValidatedJson<UpdateTechnologyRequest>,
    Extension(repo): Extension<DynRepo>,
) -> Result<impl IntoResponse, AppError> {
    Ok(pg_transaction(repo, |conn| {
        let technology_repository = TechnologyRepo::new(conn);
        let request_copy = request.clone();
        let old_data = match technology_repository.find_one(id) {
            Err(err) => {
                return Ok(server_error_response(err));
            }
            Ok(value_opt) => match value_opt {
                Some(value) => value,
                None => {
                    return Ok(not_found_response("Link"));
                }
            },
        };
        let updated_technology = UpdateTechnology { name: request.name };
        let technology_result = match technology_repository.update_one(id, &updated_technology) {
            Err(err) => {
                return Ok(server_error_response(err));
            }
            Ok(value) => value,
        };
        match create_update_admin_log(
            id.to_string(),
            claims.user_id(),
            String::from("Project Technology"),
            String::from("technologies"),
            &request_copy,
            &old_data,
            String::from("/technologies"),
            conn,
        ) {
            Ok(_) => {}
            Err(err) => {
                return Ok(server_error_response(err));
            }
        };
        Ok(simple_created_response(technology_result))
    })
    .await?)
}

pub async fn create(
    AdminClaimsContext { claims }: AdminClaimsContext,
    ValidatedJson(request): ValidatedJson<CreateTechnologyRequest>,
    Extension(repo): Extension<DynRepo>,
) -> Result<impl IntoResponse, AppError> {
    Ok(pg_transaction(repo, |conn| {
        let new_technology = NewTechnology { name: request.name };
        let new_technology_copy = new_technology.clone();
        let technology_repository = TechnologyRepo::new(conn);
        let technology_result = match technology_repository.insert_one(&new_technology) {
            Err(err) => {
                return Ok(server_error_response(err));
            }
            Ok(value) => value,
        };
        match create_creation_admin_log(
            technology_result.to_string(),
            claims.user_id(),
            String::from("Project Technology"),
            String::from("technologies"),
            &new_technology_copy,
            String::from("/technologies"),
            conn,
        ) {
            Ok(_) => {}
            Err(err) => {
                return Ok(server_error_response(err));
            }
        };
        Ok(simple_created_response(technology_result))
    })
    .await?)
}
