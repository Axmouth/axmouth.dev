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
    change_sets::UpdateTextBody,
    filters::GetAllTextBodiesFilter,
    insertables::NewTextBody,
    models::{
        queries::GetAllTextBodiesQuery,
        requests::{CreateTextBodyRequest, UpdateTextBodyRequest},
    },
};
use backend_repo_pg::{options::PaginationOptions, text_bodies::TextBodyRepo};
use chrono::Utc;

pub async fn get(slug: String, state: AppState) -> Result<impl warp::Reply, warp::Rejection> {
    let text_body_repository = TextBodyRepo::new(state.repo.clone());
    let text_body_result = match text_body_repository.find_one_by_slug(slug).await {
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
    let text_body_repository = TextBodyRepo::new(state.repo.clone());
    let (text_bodies_list, total_results) = match text_body_repository
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
    claims: Claims,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let text_body_repository = TextBodyRepo::new(state.repo.clone());
    let old_entity = match text_body_repository.find_one_by_slug(slug).await {
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
    let text_body_result = match text_body_repository.delete_one(old_entity.id).await {
        Err(err) => {
            return Ok(server_error_response(err));
        }
        Ok(value) => value,
    };
    if text_body_result == 0 {
        return Ok(not_found_response("TextBody"));
    }
    match create_deletion_admin_log(
        old_entity.id.to_string(),
        claims.user_id(),
        String::from("Text Body"),
        String::from("text_bodies"),
        &old_entity,
        String::from("/api/v1/text-bodies"),
        state.repo.clone(),
    )
    .await
    {
        Ok(_) => {}
        Err(err) => {
            return Ok(server_error_response(err));
        }
    };
    Ok(simple_no_content_response(text_body_result))
}

pub async fn update(
    slug: String,
    claims: Claims,
    request: UpdateTextBodyRequest,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let text_body_repository = TextBodyRepo::new(state.repo.clone());
    let request_copy = request.clone();
    let old_entity = match text_body_repository.find_one_by_slug(slug).await {
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
        slug: request.slug.clone(),
        title: request.title,
        url_used: request.url_used,
        updated_at: Some(Utc::now().naive_utc()),
    };
    let text_body_result = match text_body_repository
        .update_one(old_entity.id, &updated_text_body)
        .await
    {
        Err(err) => {
            return Ok(server_error_response(err));
        }
        Ok(value) => value,
    };
    match create_update_admin_log(
        old_entity.id.to_string(),
        claims.user_id(),
        String::from("Text Body"),
        String::from("text_bodies"),
        &request_copy,
        &old_entity,
        String::from("/api/v1/text-bodies"),
        state.repo.clone(),
    )
    .await
    {
        Ok(_) => {}
        Err(err) => {
            return Ok(server_error_response(err));
        }
    };
    Ok(simple_created_response(text_body_result))
}

pub async fn create(
    claims: Claims,
    request: CreateTextBodyRequest,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let new_text_body = NewTextBody {
        body: request.body,
        slug: request.slug,
        title: request.title,
        url_used: request.url_used,
    };
    let new_text_body_copy = new_text_body.clone();
    let text_body_repository = TextBodyRepo::new(state.repo.clone());
    let text_body_result = match text_body_repository.insert_one(&new_text_body).await {
        Err(err) => {
            return Ok(server_error_response(err));
        }
        Ok(value) => value,
    };
    match create_creation_admin_log(
        text_body_result.id.to_string(),
        claims.user_id(),
        String::from("Text Body"),
        String::from("text_bodies"),
        &new_text_body_copy,
        String::from("/api/v1/text-bodies"),
        state.repo.clone(),
    )
    .await
    {
        Ok(_) => {}
        Err(err) => {
            return Ok(server_error_response(err));
        }
    };
    Ok(simple_created_response(text_body_result))
}
