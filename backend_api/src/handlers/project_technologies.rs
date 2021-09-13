use crate::app::AppState;
use crate::util::create_creation_admin_log;
use crate::util::create_deletion_admin_log;
use crate::util::create_update_admin_log;
use crate::{
    auth_tokens,
    util::{
        not_found_response, paginated_ok_response, server_error_response, simple_created_response,
        simple_no_content_response, simple_ok_response,
    },
};
use auth_tokens::Claims;
use backend_repo_pg::errors::PgRepoError;
use backend_repo_pg::pg_util::RepoConnection;
use backend_repo_pg::{
    change_sets::UpdateTechnology,
    filters::GetAllTechnologiesFilter,
    insertables::NewTechnology,
    models::{
        queries::GetAllTechnologiesQuery,
        requests::{CreateTechnologyRequest, UpdateTechnologyRequest},
    },
};
use backend_repo_pg::{options::PaginationOptions, technologies::TechnologyRepo};
use tokio::task::block_in_place;

pub async fn get(id: i32, state: AppState) -> Result<impl warp::Reply, warp::Rejection> {
    block_in_place(|| {
        let conn = RepoConnection::new(state.repo)?;
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
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    block_in_place(|| {
        let conn = RepoConnection::new(state.repo)?;
        let filter = GetAllTechnologiesFilter::from_query(query.clone());
        let technology_repository = TechnologyRepo::new(&conn);
        let (technologies_list, total_results) = technology_repository
            .find(
                filter,
                query.sort_type,
                PaginationOptions {
                    page: query.page,
                    page_size: query.page_size,
                },
            )
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
    id: i32,
    claims: Claims,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(state
        .repo
        .transaction(|conn| {
            let technology_repository = TechnologyRepo::new(&conn);
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
                state.repo.clone(),
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
    id: i32,
    claims: Claims,
    request: UpdateTechnologyRequest,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(state
        .repo
        .transaction(|conn| {
            let technology_repository = TechnologyRepo::new(&conn);
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
            let technology_result = match technology_repository.update_one(id, &updated_technology)
            {
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
                state.repo.clone(),
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
    claims: Claims,
    request: CreateTechnologyRequest,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(state
        .repo
        .transaction(|conn| {
            let new_technology = NewTechnology { name: request.name };
            let new_technology_copy = new_technology.clone();
            let technology_repository = TechnologyRepo::new(&conn);
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
                state.repo.clone(),
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
