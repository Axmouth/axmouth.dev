use axum::body::BoxBody;
use axum::http::{Response, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use backend_repo_pg::pg_util::RepoConnection;
use backend_repo_pg::{
    admin_logs::AdminLogRepo, errors::PgRepoError, extra::AdminLogAction,
    insertables::NewRefreshToken, refresh_tokens::RefreshTokenRepo,
};
use backend_repo_pg::{
    insertables::NewAdminLog,
    models::responses::{AuthSuccess, BaseResponse, FileUploadedResponse, Pagination},
};
use chrono::{Duration, Utc};
use serde::Serialize;
use time::OffsetDateTime;
use tower_cookies::{Cookie, Cookies};

pub fn simple_error_response<S>(error_message: S, status: StatusCode) -> Response<BoxBody>
where
    S: Into<String>,
{
    let resp_body = BaseResponse::<()> {
        data: None,
        messages: None,
        pagination: None,
        errors: Some(vec![error_message.into()]),
        success: Some(false),
    };

    (status, Json(resp_body)).into_response()
}

pub fn simple_internal_error_response(error_message: String) -> Response<BoxBody> {
    simple_error_response(error_message, StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn simple_ok_response<T: Serialize>(data: T) -> Response<BoxBody> {
    success_response(data, StatusCode::OK).into_response()
}

pub fn paginated_ok_response<T: Serialize>(
    data: T,
    page: Option<i64>,
    page_size: Option<i64>,
    total_results: i64,
) -> Response<BoxBody> {
    let resp_body = BaseResponse::<T> {
        data: Some(data),
        messages: None,
        pagination: Some(Pagination {
            page,
            page_size,
            total_results,
        }),
        errors: None,
        success: Some(true),
    };

    (StatusCode::OK, Json(resp_body)).into_response()
}

pub fn simple_created_response<T: Serialize>(data: T) -> Response<BoxBody> {
    success_response(data, StatusCode::CREATED).into_response()
}

pub fn simple_no_content_response<T: Serialize>(data: T) -> Response<BoxBody> {
    success_response(data, StatusCode::NO_CONTENT).into_response()
}

pub fn success_response<T: Serialize>(data: T, status: StatusCode) -> Response<BoxBody> {
    let resp_body = BaseResponse::<T> {
        data: Some(data),
        messages: None,
        pagination: None,
        errors: None,
        success: Some(true),
    };

    (status, Json(resp_body)).into_response()
}

pub fn server_error_response<E>(err: E) -> Response<BoxBody>
where
    E: ToString,
{
    let resp_body = BaseResponse::<()> {
        data: None,
        messages: None,
        pagination: None,
        errors: Some(vec![err.to_string()]),
        success: Some(false),
    };

    (StatusCode::INTERNAL_SERVER_ERROR, Json(resp_body)).into_response()
}

pub fn translated_error_response<E: std::error::Error>(
    err: E,
    code: StatusCode,
) -> Response<BoxBody> {
    let resp_body = BaseResponse::<()> {
        data: None,
        messages: None,
        pagination: None,
        errors: Some(vec![err.to_string()]),
        success: Some(false),
    };

    (code, Json(resp_body)).into_response()
}

pub fn not_found_response(entity_name: &str) -> Response<BoxBody> {
    let resp_body = BaseResponse::<()> {
        data: None,
        messages: None,
        pagination: None,
        errors: Some(vec![format!("{} not found", entity_name)]),
        success: Some(false),
    };

    (StatusCode::NOT_FOUND, Json(resp_body)).into_response()
}

pub fn unauthorized_response(entity_name: &str) -> Response<BoxBody> {
    let resp_body = BaseResponse::<()> {
        data: None,
        messages: None,
        pagination: None,
        errors: Some(vec![format!("You cannot edit this {}", entity_name)]),
        success: Some(false),
    };

    (StatusCode::UNAUTHORIZED, Json(resp_body)).into_response()
}

pub fn bad_request_response<S>(err_message: S) -> Response<BoxBody>
where
    S: Into<String>,
{
    let resp_body = BaseResponse::<()> {
        data: None,
        messages: None,
        pagination: None,
        errors: Some(vec![err_message.into()]),
        success: Some(false),
    };

    (StatusCode::BAD_REQUEST, Json(resp_body)).into_response()
}

pub fn bad_request_response_many(err_messages: Vec<String>) -> Response<BoxBody> {
    let resp_body = BaseResponse::<()> {
        data: None,
        messages: None,
        pagination: None,
        errors: Some(err_messages),
        success: Some(false),
    };

    (StatusCode::BAD_REQUEST, Json(resp_body)).into_response()
}

pub fn login_failed_response(cookies: Cookies) -> Response<BoxBody> {
    cookies.remove(Cookie::new("refresh_token", ""));
    simple_error_response(
        "No such Email/Password combination".to_string(),
        StatusCode::UNAUTHORIZED,
    )
}

pub fn auth_error_response<E: std::error::Error>(err: E, cookies: Cookies) -> Response<BoxBody> {
    let resp_body = Json(BaseResponse::<()> {
        data: None,
        messages: None,
        pagination: None,
        errors: Some(vec![err.to_string()]),
        success: Some(false),
    });

    cookies.remove(Cookie::new("refresh_token", ""));

    (StatusCode::INTERNAL_SERVER_ERROR, resp_body).into_response()
}

pub fn auth_unauthorized_response(err_message: &str, cookies: Cookies) -> Response<BoxBody> {
    let resp_body = Json(BaseResponse::<()> {
        data: None,
        messages: None,
        pagination: None,
        errors: Some(vec![err_message.to_string()]),
        success: Some(false),
    });

    cookies.remove(Cookie::new("refresh_token", ""));

    (StatusCode::UNAUTHORIZED, resp_body).into_response()
}

pub fn auth_bad_request_response(err_message: &str, cookies: Cookies) -> Response<BoxBody> {
    let resp_body = Json(BaseResponse::<()> {
        data: None,
        messages: None,
        pagination: None,
        errors: Some(vec![err_message.to_string()]),
        success: Some(false),
    });

    cookies.remove(Cookie::new("refresh_token", ""));

    (StatusCode::BAD_REQUEST, resp_body).into_response()
}

pub fn auth_ok_response(
    jwt_token: String,
    refresh_token: uuid::Uuid,
    mut refresh_cookie: Cookie<'static>,
    cookies: Cookies,
) -> Response<BoxBody> {
    let resp_body = Json(BaseResponse {
        data: Some(AuthSuccess { token: jwt_token }),
        success: Some(true),
        errors: None,
        messages: None,
        pagination: None,
    });

    let mut expiration = OffsetDateTime::now_utc();
    expiration += time::Duration::days(6 * 30);
    refresh_cookie.set_expires(expiration);
    refresh_cookie.set_value(refresh_token.to_string());
    cookies.add(refresh_cookie);

    (StatusCode::CREATED, resp_body).into_response()
}

pub fn upload_bad_request_response(err_message: &str) -> Response<BoxBody> {
    let resp_body = Json(FileUploadedResponse {
        success: 0,
        file: None,
        errors: Some(vec![err_message.to_string()]),
    });
    (StatusCode::BAD_REQUEST, resp_body).into_response()
}

pub fn upload_error_response<E: std::error::Error>(err: E) -> Response<BoxBody> {
    let resp_body = Json(FileUploadedResponse {
        success: 0,
        file: None,
        errors: Some(vec![err.to_string()]),
    });
    (StatusCode::INTERNAL_SERVER_ERROR, resp_body).into_response()
}

pub fn create_refresh_token(
    user_id: i32,
    jwt_id: uuid::Uuid,
    repo: RefreshTokenRepo<'_>,
) -> Result<uuid::Uuid, PgRepoError> {
    let new_token = NewRefreshToken {
        jwt_id,
        user_id,
        invalidated: false,
        used: false,
        expires_at: (Utc::now() + Duration::days(30 * 6)).naive_utc(),
    };
    Ok(repo.insert_one(new_token)?.id)
}

#[allow(clippy::too_many_arguments)]
fn create_admin_log<T, U>(
    object_id: String,
    user_id: i32,
    label: String,
    model: String,
    action: AdminLogAction,
    new_data: Option<&T>,
    old_data: Option<&U>,
    base_link: String,
    conn: &RepoConnection,
) -> Result<(), PgRepoError>
where
    T: Serialize,
    U: Serialize,
{
    let new_data = new_data.map(|v| serde_json::to_string(v).unwrap_or_default());
    let old_data = old_data.map(|v| serde_json::to_string(v).unwrap_or_default());
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

    let admin_log_repository = AdminLogRepo::new(conn);
    admin_log_repository.insert_one(new_admin_log)?;

    Ok(())
}

pub fn create_creation_admin_log<T>(
    object_id: String,
    user_id: i32,
    label: String,
    model: String,
    new_data: &T,
    base_link: String,
    conn: &RepoConnection,
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
        conn,
    )?;

    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub fn create_update_admin_log<T, U>(
    object_id: String,
    user_id: i32,
    label: String,
    model: String,
    new_data: &T,
    old_data: &U,
    base_link: String,
    conn: &RepoConnection,
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
        conn,
    )?;

    Ok(())
}

pub fn create_deletion_admin_log<T>(
    object_id: String,
    user_id: i32,
    label: String,
    model: String,
    old_data: &T,
    base_link: String,
    conn: &RepoConnection,
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
        conn,
    )?;

    Ok(())
}
