use crate::app::AppState;
use crate::{
    auth_tokens,
    util::{
        not_found_response, paginated_ok_response, server_error_response, simple_created_response,
        simple_no_content_response, simple_ok_response,
    },
};
use auth_tokens::Claims;
use backend_repo_pg::options::{
    HomePageLinkSort, HomePageLinkSortType, PaginationOptions, SortOrder,
};
use backend_repo_pg::{
    change_sets::UpdateHomePageLink,
    filters::GetAllHomePageLinksFilter,
    insertables::NewHomePageLink,
    models::{
        queries::GetAllHomePageLinksQuery,
        requests::{CreateHomePageLinkRequest, UpdateHomePageLinkRequest},
    },
};
use chrono::Utc;

pub async fn get(id: i32, state: AppState) -> Result<impl warp::Reply, warp::Rejection> {
    let link_result = match state.repository.link_repository.find_one(id).await {
        Err(err) => {
            return Ok(server_error_response(err));
        }
        Ok(value_opt) => match value_opt {
            None => {
                return Ok(not_found_response("HomePageLink"));
            }
            Some(value) => value,
        },
    };
    Ok(simple_ok_response(link_result))
}

pub async fn get_all(
    query: GetAllHomePageLinksQuery,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let filter = GetAllHomePageLinksFilter::from_query(query.clone());
    let (links_list, total_results) = match state
        .repository
        .link_repository
        .find(
            filter,
            HomePageLinkSort {
                order: None,
                sort_type: None,
            },
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
        links_list,
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
    let _ = match state.repository.link_repository.find_one(id).await {
        Err(err) => {
            return Ok(server_error_response(err));
        }
        Ok(value_opt) => match value_opt {
            None => {
                return Ok(not_found_response("HomePageLink"));
            }
            Some(value) => value,
        },
    };
    let link_result = match state.repository.link_repository.delete_one(id).await {
        Err(err) => {
            return Ok(server_error_response(err));
        }
        Ok(value) => value,
    };
    if link_result == 0 {
        return Ok(not_found_response("HomePageLink"));
    }
    Ok(simple_no_content_response(link_result))
}

pub async fn update(
    id: i32,
    _claims: Claims,
    request: UpdateHomePageLinkRequest,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let _ = match state.repository.link_repository.find_one(id).await {
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
    let updated_link = UpdateHomePageLink {
        name: request.name,
        image: request.image,
        target: request.target,
    };
    let link_result = match state
        .repository
        .link_repository
        .update_one(id, updated_link)
        .await
    {
        Err(err) => {
            return Ok(server_error_response(err));
        }
        Ok(value) => value,
    };
    Ok(simple_created_response(link_result))
}

pub async fn create(
    _claims: Claims,
    request: CreateHomePageLinkRequest,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let new_link = NewHomePageLink {
        name: request.name,
        image: request.image,
        target: request.target,
    };
    let link_result = match state.repository.link_repository.insert_one(new_link).await {
        Err(err) => {
            return Ok(server_error_response(err));
        }
        Ok(value) => value,
    };
    Ok(simple_created_response(link_result))
}
