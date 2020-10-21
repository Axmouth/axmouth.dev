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
    change_sets::UpdateTextBody,
    filters::GetAllTextBodiesFilter,
    insertables::NewTextBody,
    models::{
        queries::GetAllTextBodiesQuery,
        requests::{CreateTextBodyRequest, UpdateTextBodyRequest},
    },
};
use chrono::Utc;

pub async fn get(slug: String, state: AppState) -> Result<impl warp::Reply, warp::Rejection> {
    let text_body_result = match state
        .repository
        .text_body_repository
        .find_one_by_slug(slug)
        .await
    {
        Err(err) => {
            return Ok(server_error_response(err));
        }
        Ok(value_opt) => match value_opt {
            None => {
                return Ok(not_found_response("TextBody"));
            }
            Some(value) => value,
        },
    };
    Ok(simple_ok_response(text_body_result))
}

pub async fn get_all(
    query: GetAllTextBodiesQuery,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let filter = GetAllTextBodiesFilter::from_query(query.clone());
    let (text_bodies_list, total_results) = match state
        .repository
        .text_body_repository
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
        text_bodies_list,
        query.page,
        query.page_size,
        total_results,
    ))
}

pub async fn delete(
    slug: String,
    _claims: Claims,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let old_entity = match state
        .repository
        .text_body_repository
        .find_one_by_slug(slug)
        .await
    {
        Err(err) => {
            return Ok(server_error_response(err));
        }
        Ok(value_opt) => match value_opt {
            None => {
                return Ok(not_found_response("TextBody"));
            }
            Some(value) => value,
        },
    };
    let text_body_result = match state
        .repository
        .text_body_repository
        .delete_one(old_entity.id)
        .await
    {
        Err(err) => {
            return Ok(server_error_response(err));
        }
        Ok(value) => value,
    };
    if text_body_result == 0 {
        return Ok(not_found_response("TextBody"));
    }
    Ok(simple_no_content_response(text_body_result))
}

pub async fn update(
    slug: String,
    _claims: Claims,
    request: UpdateTextBodyRequest,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let old_entity = match state
        .repository
        .text_body_repository
        .find_one_by_slug(slug)
        .await
    {
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
    let updated_text_body = UpdateTextBody {
        body: request.body,
        slug: request.slug,
        title: request.title,
        url_used: request.url_used,
        updated_at: Some(Utc::now().naive_utc()),
    };
    let text_body_result = match state
        .repository
        .text_body_repository
        .update_one(old_entity.id, updated_text_body)
        .await
    {
        Err(err) => {
            return Ok(server_error_response(err));
        }
        Ok(value) => value,
    };
    Ok(simple_created_response(text_body_result))
}

pub async fn create(
    _claims: Claims,
    request: CreateTextBodyRequest,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let new_text_body = NewTextBody {
        body: request.body,
        slug: request.slug,
        title: request.title,
        url_used: request.url_used,
    };
    let text_body_result = match state
        .repository
        .text_body_repository
        .insert_one(new_text_body)
        .await
    {
        Err(err) => {
            return Ok(server_error_response(err));
        }
        Ok(value) => value,
    };
    Ok(simple_created_response(text_body_result))
}
