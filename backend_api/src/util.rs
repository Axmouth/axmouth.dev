use crate::cookies::CookieBuilder;
use backend_repo_pg::models::responses::{
    AuthSuccess, BaseResponse, FileUploadedResponse, Pagination,
};
use backend_repo_pg::{
    errors::PgRepoError, insertables::NewRefreshToken, refresh_tokens::RefreshTokenRepo,
};
use chrono::{Duration, Utc};
use serde::Serialize;
use warp::hyper::header;
use warp::hyper::StatusCode;

pub fn simple_error_response(
    error_message: String,
    status: StatusCode,
) -> warp::reply::WithStatus<warp::reply::Json> {
    let resp_body = warp::reply::json(&BaseResponse::<()> {
        data: None,
        messages: None,
        pagination: None,
        errors: Some(vec![error_message]),
        success: Some(false),
    });
    warp::reply::with_status(resp_body, status)
}

pub fn simple_internal_error_response(
    error_message: String,
) -> warp::reply::WithStatus<warp::reply::Json> {
    simple_error_response(error_message, StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn simple_ok_response<T: Serialize>(data: T) -> warp::reply::WithStatus<warp::reply::Json> {
    success_response(data, StatusCode::OK)
}

pub fn paginated_ok_response<T: Serialize>(
    data: T,
    page: Option<i64>,
    page_size: Option<i64>,
    total_results: i64,
) -> warp::reply::WithStatus<warp::reply::Json> {
    let resp_body = warp::reply::json(&BaseResponse::<T> {
        data: Some(data),
        messages: None,
        pagination: Some(Pagination {
            page,
            page_size,
            total_results,
        }),
        errors: None,
        success: Some(true),
    });
    warp::reply::with_status(resp_body, StatusCode::OK)
}

pub fn simple_created_response<T: Serialize>(
    data: T,
) -> warp::reply::WithStatus<warp::reply::Json> {
    success_response(data, StatusCode::CREATED)
}

pub fn simple_no_content_response<T: Serialize>(
    data: T,
) -> warp::reply::WithStatus<warp::reply::Json> {
    success_response(data, StatusCode::NO_CONTENT)
}

pub fn success_response<T: Serialize>(
    data: T,
    status: StatusCode,
) -> warp::reply::WithStatus<warp::reply::Json> {
    let resp_body = warp::reply::json(&BaseResponse::<T> {
        data: Some(data),
        messages: None,
        pagination: None,
        errors: None,
        success: Some(true),
    });
    warp::reply::with_status(resp_body, status)
}

pub fn server_error_response<E: std::error::Error>(
    err: E,
) -> warp::reply::WithStatus<warp::reply::Json> {
    let resp_body = warp::reply::json(&BaseResponse::<()> {
        data: None,
        messages: None,
        pagination: None,
        errors: Some(vec![err.to_string()]),
        success: Some(false),
    });
    warp::reply::with_status(resp_body, StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn not_found_response(entity_name: &str) -> warp::reply::WithStatus<warp::reply::Json> {
    let resp_body = warp::reply::json(&BaseResponse::<()> {
        data: None,
        messages: None,
        pagination: None,
        errors: Some(vec![format!("{} not found", entity_name)]),
        success: Some(false),
    });
    warp::reply::with_status(resp_body, StatusCode::NOT_FOUND)
}

pub fn unauthorized_response(entity_name: &str) -> warp::reply::WithStatus<warp::reply::Json> {
    let resp_body = warp::reply::json(&BaseResponse::<()> {
        data: None,
        messages: None,
        pagination: None,
        errors: Some(vec![format!("You cannot edit this {}", entity_name)]),
        success: Some(false),
    });
    warp::reply::with_status(resp_body, StatusCode::UNAUTHORIZED)
}

pub fn bad_request_response(err_message: &str) -> warp::reply::WithStatus<warp::reply::Json> {
    let resp_body = warp::reply::json(&BaseResponse::<()> {
        data: None,
        messages: None,
        pagination: None,
        errors: Some(vec![err_message.to_string()]),
        success: Some(false),
    });
    warp::reply::with_status(resp_body, StatusCode::BAD_REQUEST)
}

pub fn login_failed_response() -> warp::reply::WithHeader<warp::reply::WithStatus<warp::reply::Json>>
{
    let resp_with_status = simple_error_response(
        "No such Email/Password combination".to_string(),
        StatusCode::UNAUTHORIZED,
    );

    warp::reply::with_header(resp_with_status, header::SET_COOKIE, "refresh_token=")
}

pub fn auth_error_response<E: std::error::Error>(
    err: E,
) -> warp::reply::WithHeader<warp::reply::WithStatus<warp::reply::Json>> {
    let resp_body = warp::reply::json(&BaseResponse::<()> {
        data: None,
        messages: None,
        pagination: None,
        errors: Some(vec![err.to_string()]),
        success: Some(false),
    });
    let resp_with_status = warp::reply::with_status(resp_body, StatusCode::INTERNAL_SERVER_ERROR);

    warp::reply::with_header(resp_with_status, header::SET_COOKIE, "refresh_token=")
}

pub fn auth_unauthorized_response(
    err_message: &str,
) -> warp::reply::WithHeader<warp::reply::WithStatus<warp::reply::Json>> {
    let resp_body = warp::reply::json(&BaseResponse::<()> {
        data: None,
        messages: None,
        pagination: None,
        errors: Some(vec![err_message.to_string()]),
        success: Some(false),
    });
    let resp_with_status = warp::reply::with_status(resp_body, StatusCode::UNAUTHORIZED);

    warp::reply::with_header(resp_with_status, header::SET_COOKIE, "refresh_token=")
}

pub fn auth_bad_request_response(
    err_message: &str,
) -> warp::reply::WithHeader<warp::reply::WithStatus<warp::reply::Json>> {
    let resp_body = warp::reply::json(&BaseResponse::<()> {
        data: None,
        messages: None,
        pagination: None,
        errors: Some(vec![err_message.to_string()]),
        success: Some(false),
    });
    let resp_with_status = warp::reply::with_status(resp_body, StatusCode::BAD_REQUEST);

    warp::reply::with_header(resp_with_status, header::SET_COOKIE, "refresh_token=")
}

pub fn auth_ok_response(
    jwt_token: String,
    refresh_token: uuid::Uuid,
    refresh_cookie_builder: CookieBuilder,
) -> warp::reply::WithHeader<warp::reply::WithStatus<warp::reply::Json>> {
    let resp_body = warp::reply::json(&BaseResponse {
        data: Some(AuthSuccess {
            token: jwt_token.clone(),
        }),
        success: Some(true),
        errors: None,
        messages: None,
        pagination: None,
    });
    let value = "df".to_string();
    value
        .split_terminator(':')
        .collect::<Vec<&str>>()
        .get(1 as usize)
        .map_or("", |val| val)
        .to_string();
    let resp_with_status = warp::reply::with_status(resp_body, StatusCode::CREATED);
    let refresh_token_cookie = refresh_token.to_string();
    refresh_cookie_builder
        .cookie_with_value_and_expires_days(resp_with_status, refresh_token_cookie, 6 * 30)
        .unwrap()
}

pub fn upload_bad_request_response(
    err_message: &str,
) -> warp::reply::WithStatus<warp::reply::Json> {
    let resp_body = warp::reply::json(&FileUploadedResponse {
        success: 0,
        file: None,
        errors: Some(vec![err_message.to_string()]),
    });
    warp::reply::with_status(resp_body, StatusCode::BAD_REQUEST)
}

pub fn upload_error_response<E: std::error::Error>(
    err: E,
) -> warp::reply::WithStatus<warp::reply::Json> {
    let resp_body = warp::reply::json(&FileUploadedResponse {
        success: 0,
        file: None,
        errors: Some(vec![err.to_string()]),
    });
    warp::reply::with_status(resp_body, StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn create_refresh_token(
    user_id: i32,
    jwt_id: uuid::Uuid,
    repo: RefreshTokenRepo,
) -> Result<uuid::Uuid, PgRepoError> {
    let new_token = NewRefreshToken {
        jwt_id,
        user_id,
        invalidated: false,
        used: false,
        expires_at: (Utc::now() + Duration::days(30 * 6)).naive_utc(),
    };
    Ok(repo.insert_one(new_token).await?.id)
}
