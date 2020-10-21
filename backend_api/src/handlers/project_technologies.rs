use crate::app::AppState;
use crate::{
    auth_tokens,
    util::{
        not_found_response, paginated_ok_response, server_error_response, simple_created_response,
        simple_no_content_response, simple_ok_response,
    },
};
use auth_tokens::Claims;
use backend_repo_pg::options::PaginationOptions;
use backend_repo_pg::{
    change_sets::UpdateTechnology,
    filters::GetAllTechnologiesFilter,
    insertables::NewTechnology,
    models::{
        queries::GetAllTechnologiesQuery,
        requests::{CreateTechnologyRequest, UpdateTechnologyRequest},
    },
};

pub async fn get(id: i32, state: AppState) -> Result<impl warp::Reply, warp::Rejection> {
    let technology_result = match state.repository.technology_repository.find_one(id).await {
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
    Ok(simple_ok_response(technology_result))
}

pub async fn get_all(
    query: GetAllTechnologiesQuery,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let filter = GetAllTechnologiesFilter::from_query(query.clone());
    let (technologies_list, total_results) = match state
        .repository
        .technology_repository
        .find(
            filter,
            query.sort_type,
            PaginationOptions {
                page: query.page,
                page_size: query.page_size,
            },
        )
        .await
    {
        Err(err) => {
            return Ok(server_error_response(err));
        }
        Ok(value) => value,
    };
    Ok(paginated_ok_response(
        technologies_list,
        query.page,
        query.page_size,
        total_results,
    ))
}

pub async fn delete(
    id: i32,
    _claims: Claims,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let _ = match state.repository.technology_repository.find_one(id).await {
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
    let technology_result = match state.repository.technology_repository.delete_one(id).await {
        Err(err) => {
            return Ok(server_error_response(err));
        }
        Ok(value) => value,
    };
    if technology_result == 0 {
        return Ok(not_found_response("Technology"));
    }
    Ok(simple_no_content_response(technology_result))
}

pub async fn update(
    id: i32,
    _claims: Claims,
    request: UpdateTechnologyRequest,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let _ = match state.repository.technology_repository.find_one(id).await {
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
    let technology_result = match state
        .repository
        .technology_repository
        .update_one(id, updated_technology)
        .await
    {
        Err(err) => {
            return Ok(server_error_response(err));
        }
        Ok(value) => value,
    };
    Ok(simple_created_response(technology_result))
}

pub async fn create(
    _claims: Claims,
    request: CreateTechnologyRequest,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let new_technology = NewTechnology { name: request.name };
    let technology_result = match state
        .repository
        .technology_repository
        .insert_one(new_technology)
        .await
    {
        Err(err) => {
            return Ok(server_error_response(err));
        }
        Ok(value) => value,
    };
    Ok(simple_created_response(technology_result))
}
