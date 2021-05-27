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
use backend_repo_pg::{
    change_sets::UpdateHomePageLink,
    filters::GetAllHomePageLinksFilter,
    insertables::NewHomePageLink,
    models::{
        queries::GetAllHomePageLinksQuery,
        requests::{CreateHomePageLinkRequest, UpdateHomePageLinkRequest},
    },
};
use backend_repo_pg::{home_page_links::HomePageLinkRepo, options::PaginationOptions};

pub async fn get(id: i32, state: AppState) -> Result<impl warp::Reply, warp::Rejection> {
    let link_repository = HomePageLinkRepo::new(state.repo.clone());
    let link_result = match link_repository.find_one(id).await {
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
    let link_repository = HomePageLinkRepo::new(state.repo.clone());
    let (links_list, total_results) = match link_repository
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
        links_list,
        query.page,
        query.page_size,
        total_results,
    ))
}

pub async fn delete(
    id: i32,
    claims: Claims,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let link_repository = HomePageLinkRepo::new(state.repo.clone());
    let old_data = match link_repository.find_one(id).await {
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
    let link_result = match link_repository.delete_one(id).await {
        Err(err) => {
            return Ok(server_error_response(err));
        }
        Ok(value) => value,
    };
    if link_result == 0 {
        return Ok(not_found_response("HomePageLink"));
    }
    match create_deletion_admin_log(
        id.to_string(),
        claims.user_id(),
        String::from("Link"),
        String::from("home_page_links"),
        &old_data,
        String::from("/api/v1/links"),
        state.repo.clone(),
    )
    .await
    {
        Ok(_) => {}
        Err(err) => {
            return Ok(server_error_response(err));
        }
    };
    Ok(simple_no_content_response(link_result))
}

pub async fn update(
    id: i32,
    claims: Claims,
    request: UpdateHomePageLinkRequest,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let link_repository = HomePageLinkRepo::new(state.repo.clone());
    let request_copy = request.clone();
    let old_data = match link_repository.find_one(id).await {
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
    let link_result = match link_repository.update_one(id, &updated_link).await {
        Err(err) => {
            return Ok(server_error_response(err));
        }
        Ok(value) => value,
    };
    match create_update_admin_log(
        id.to_string(),
        claims.user_id(),
        String::from("Link"),
        String::from("home_page_links"),
        &request_copy,
        &old_data,
        String::from("/api/v1/links"),
        state.repo.clone(),
    )
    .await
    {
        Ok(_) => {}
        Err(err) => {
            return Ok(server_error_response(err));
        }
    };
    Ok(simple_created_response(link_result))
}

pub async fn create(
    claims: Claims,
    request: CreateHomePageLinkRequest,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let new_link = NewHomePageLink {
        name: request.name,
        image: request.image,
        target: request.target,
    };
    let new_link_copy = new_link.clone();
    let link_repository = HomePageLinkRepo::new(state.repo.clone());
    let link_result = match link_repository.insert_one(new_link).await {
        Err(err) => {
            return Ok(server_error_response(err));
        }
        Ok(value) => value,
    };
    match create_creation_admin_log(
        link_result.to_string(),
        claims.user_id(),
        String::from("Link"),
        String::from("home_page_links"),
        &new_link_copy,
        String::from("/api/v1/links"),
        state.repo.clone(),
    )
    .await
    {
        Ok(_) => {}
        Err(err) => {
            return Ok(server_error_response(err));
        }
    };
    Ok(simple_created_response(link_result))
}
