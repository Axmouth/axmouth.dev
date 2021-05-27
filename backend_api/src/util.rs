use crate::cookies::CookieBuilder;
use backend_repo_pg::{
    admin_logs::AdminLogRepo, errors::PgRepoError, extra::AdminLogAction,
    insertables::NewRefreshToken, pg_util::Repo, refresh_tokens::RefreshTokenRepo,
};
use backend_repo_pg::{
    insertables::NewAdminLog,
    models::responses::{AuthSuccess, BaseResponse, FileUploadedResponse, Pagination},
};
use chrono::{Duration, Utc};
use serde::Serialize;
use warp::hyper::StatusCode;
use warp::{hyper::header, Reply};

pub fn simple_error_response(error_message: String, status: StatusCode) -> warp::reply::Response {
    let resp_body = warp::reply::json(&BaseResponse::<()> {
        data: None,
        messages: None,
        pagination: None,
        errors: Some(vec![error_message]),
        success: Some(false),
    });
    warp::reply::with_status(resp_body, status).into_response()
}

pub fn simple_internal_error_response(error_message: String) -> warp::reply::Response {
    simple_error_response(error_message, StatusCode::INTERNAL_SERVER_ERROR).into_response()
}

pub fn simple_ok_response<T: Serialize>(data: T) -> warp::reply::Response {
    success_response(data, StatusCode::OK).into_response()
}

pub fn paginated_ok_response<T: Serialize>(
    data: T,
    page: Option<i64>,
    page_size: Option<i64>,
    total_results: i64,
) -> warp::reply::Response {
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
    warp::reply::with_status(resp_body, StatusCode::OK).into_response()
}

pub fn simple_created_response<T: Serialize>(data: T) -> warp::reply::Response {
    success_response(data, StatusCode::CREATED).into_response()
}

pub fn simple_no_content_response<T: Serialize>(data: T) -> warp::reply::Response {
    success_response(data, StatusCode::NO_CONTENT).into_response()
}

pub fn success_response<T: Serialize>(data: T, status: StatusCode) -> warp::reply::Response {
    let resp_body = warp::reply::json(&BaseResponse::<T> {
        data: Some(data),
        messages: None,
        pagination: None,
        errors: None,
        success: Some(true),
    });
    warp::reply::with_status(resp_body, status).into_response()
}

pub fn server_error_response<E: std::error::Error>(err: E) -> warp::reply::Response {
    let resp_body = warp::reply::json(&BaseResponse::<()> {
        data: None,
        messages: None,
        pagination: None,
        errors: Some(vec![err.to_string()]),
        success: Some(false),
    });
    warp::reply::with_status(resp_body, StatusCode::INTERNAL_SERVER_ERROR).into_response()
}

pub fn translated_error_response<E: std::error::Error>(
    err: E,
    code: StatusCode,
) -> warp::reply::Response {
    let resp_body = warp::reply::json(&BaseResponse::<()> {
        data: None,
        messages: None,
        pagination: None,
        errors: Some(vec![err.to_string()]),
        success: Some(false),
    });
    warp::reply::with_status(resp_body, code).into_response()
}

pub fn not_found_response(entity_name: &str) -> warp::reply::Response {
    let resp_body = warp::reply::json(&BaseResponse::<()> {
        data: None,
        messages: None,
        pagination: None,
        errors: Some(vec![format!("{} not found", entity_name)]),
        success: Some(false),
    });
    warp::reply::with_status(resp_body, StatusCode::NOT_FOUND).into_response()
}

pub fn unauthorized_response(entity_name: &str) -> warp::reply::Response {
    let resp_body = warp::reply::json(&BaseResponse::<()> {
        data: None,
        messages: None,
        pagination: None,
        errors: Some(vec![format!("You cannot edit this {}", entity_name)]),
        success: Some(false),
    });
    warp::reply::with_status(resp_body, StatusCode::UNAUTHORIZED).into_response()
}

pub fn bad_request_response(err_message: &str) -> warp::reply::Response {
    let resp_body = warp::reply::json(&BaseResponse::<()> {
        data: None,
        messages: None,
        pagination: None,
        errors: Some(vec![err_message.to_string()]),
        success: Some(false),
    });
    warp::reply::with_status(resp_body, StatusCode::BAD_REQUEST).into_response()
}

pub fn login_failed_response() -> warp::reply::Response {
    let resp_with_status = simple_error_response(
        "No such Email/Password combination".to_string(),
        StatusCode::UNAUTHORIZED,
    );

    warp::reply::with_header(resp_with_status, header::SET_COOKIE, "refresh_token=").into_response()
}

pub fn auth_error_response<E: std::error::Error>(err: E) -> warp::reply::Response {
    let resp_body = warp::reply::json(&BaseResponse::<()> {
        data: None,
        messages: None,
        pagination: None,
        errors: Some(vec![err.to_string()]),
        success: Some(false),
    });
    let resp_with_status = warp::reply::with_status(resp_body, StatusCode::INTERNAL_SERVER_ERROR);

    warp::reply::with_header(resp_with_status, header::SET_COOKIE, "refresh_token=").into_response()
}

pub fn auth_unauthorized_response(err_message: &str) -> warp::reply::Response {
    let resp_body = warp::reply::json(&BaseResponse::<()> {
        data: None,
        messages: None,
        pagination: None,
        errors: Some(vec![err_message.to_string()]),
        success: Some(false),
    });
    let resp_with_status = warp::reply::with_status(resp_body, StatusCode::UNAUTHORIZED);

    warp::reply::with_header(resp_with_status, header::SET_COOKIE, "refresh_token=").into_response()
}

pub fn auth_bad_request_response(err_message: &str) -> warp::reply::Response {
    let resp_body = warp::reply::json(&BaseResponse::<()> {
        data: None,
        messages: None,
        pagination: None,
        errors: Some(vec![err_message.to_string()]),
        success: Some(false),
    });
    let resp_with_status = warp::reply::with_status(resp_body, StatusCode::BAD_REQUEST);

    warp::reply::with_header(resp_with_status, header::SET_COOKIE, "refresh_token=").into_response()
}

pub fn auth_ok_response(
    jwt_token: String,
    refresh_token: uuid::Uuid,
    refresh_cookie_builder: CookieBuilder,
) -> warp::reply::Response {
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
        .into_response()
}

pub fn upload_bad_request_response(err_message: &str) -> warp::reply::Response {
    let resp_body = warp::reply::json(&FileUploadedResponse {
        success: 0,
        file: None,
        errors: Some(vec![err_message.to_string()]),
    });
    warp::reply::with_status(resp_body, StatusCode::BAD_REQUEST).into_response()
}

pub fn upload_error_response<E: std::error::Error>(err: E) -> warp::reply::Response {
    let resp_body = warp::reply::json(&FileUploadedResponse {
        success: 0,
        file: None,
        errors: Some(vec![err.to_string()]),
    });
    warp::reply::with_status(resp_body, StatusCode::INTERNAL_SERVER_ERROR).into_response()
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

async fn create_admin_log<T, U>(
    object_id: String,
    user_id: i32,
    label: String,
    model: String,
    action: AdminLogAction,
    new_data: Option<&T>,
    old_data: Option<&U>,
    base_link: String,
    repo: Repo,
) -> Result<(), PgRepoError>
where
    T: Serialize,
    U: Serialize,
{
    let new_data = new_data.map(|v| serde_json::to_string(v).unwrap_or(String::new()));
    let old_data = old_data.map(|v| serde_json::to_string(v).unwrap_or(String::new()));
    let new_admin_log = NewAdminLog {
        object_id,
        user_id,
        label,
        model,
        action,
        new_data,
        old_data,
        base_link,
    };

    let admin_log_repository = AdminLogRepo::new(repo);
    admin_log_repository.insert_one(new_admin_log).await?;

    Ok(())
}

pub async fn create_creation_admin_log<T>(
    object_id: String,
    user_id: i32,
    label: String,
    model: String,
    new_data: &T,
    base_link: String,
    repo: Repo,
) -> Result<(), PgRepoError>
where
    T: Serialize,
{
    create_admin_log(
        object_id,
        user_id,
        label,
        model,
        AdminLogAction::Create,
        Some(new_data),
        None as Option<&()>,
        base_link,
        repo,
    )
    .await?;

    Ok(())
}

pub async fn create_update_admin_log<T, U>(
    object_id: String,
    user_id: i32,
    label: String,
    model: String,
    new_data: &T,
    old_data: &U,
    base_link: String,
    repo: Repo,
) -> Result<(), PgRepoError>
where
    T: Serialize,
    U: Serialize,
{
    create_admin_log(
        object_id,
        user_id,
        label,
        model,
        AdminLogAction::Update,
        Some(new_data),
        Some(old_data),
        base_link,
        repo,
    )
    .await?;

    Ok(())
}

pub async fn create_deletion_admin_log<T>(
    object_id: String,
    user_id: i32,
    label: String,
    model: String,
    old_data: &T,
    base_link: String,
    repo: Repo,
) -> Result<(), PgRepoError>
where
    T: Serialize,
{
    create_admin_log(
        object_id,
        user_id,
        label,
        model,
        AdminLogAction::Delete,
        None as Option<&()>,
        Some(old_data),
        base_link,
        repo,
    )
    .await?;

    Ok(())
}
