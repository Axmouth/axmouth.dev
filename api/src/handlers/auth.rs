use crate::app::{DynEmailSender, DynJwtDuration, DynJwtSecret};
use crate::errors::AppError;
use crate::extractors::{ClaimsContext, ValidatedJson};
use crate::util::{
    auth_bad_request_response, auth_error_response, auth_ok_response, auth_unauthorized_response,
    bad_request_response, create_refresh_token, login_failed_response, not_found_response,
    server_error_response,
};
use crate::{auth_tokens, util::simple_error_response};
use crate::{auth_tokens::decode_token, util::simple_ok_response};
use axum::body::BoxBody;
use axum::extract::Extension;
use axum::response::{IntoResponse, Response};
use axum::Json;
use backend_repo_pg::models::domain::User;
use backend_repo_pg::pg_util::DynRepo;
use backend_repo_pg::{
    change_password_tokens::ChangePasswordTokenRepo, change_sets::UpdateChangePasswordToken,
    change_sets::UpdateUser, change_sets::UpdateVerifyEmailToken, errors::PgRepoError,
    insertables::NewChangePasswordToken, insertables::NewVerifyEmailToken,
    models::requests::RequestResetPasswordEmailRequest,
    models::requests::RequestVerificationEmailRequest, models::requests::ResetPasswordRequest,
    models::requests::VerifyEmailRequest, models::responses::BaseResponse,
    refresh_tokens::RefreshTokenRepo, verify_email_tokens::VerifyEmailTokenRepo,
};
use backend_repo_pg::{
    extra::UserRole,
    insertables::NewUser,
    models::requests::{LoginRequest, RefreshRequest, RegisterRequest},
};
use backend_repo_pg::{passwords, users::UserRepo};
use chrono::{Duration, Utc};
use hyper::StatusCode;
use rand::{distributions::Alphanumeric, Rng};
use tower_cookies::{Cookie, Cookies};

pub async fn login(
    ValidatedJson(request): ValidatedJson<LoginRequest>,
    cookies: Cookies,
    Extension(repo): Extension<DynRepo>,
    Extension(jwt_secret): Extension<DynJwtSecret>,
    Extension(jwt_duration): Extension<DynJwtDuration>,
) -> Result<impl IntoResponse, AppError> {
    let conn = repo.get_async_conn();

    let user_repository = UserRepo::new(&conn);
    let user = match user_repository.find_one_by_email(request.email).await? {
        None => {
            return Ok(login_failed_response(cookies));
        }
        Some(value) => value,
    };

    let password_match = passwords::verify(user.password.as_str(), request.password.as_bytes());

    if !password_match {
        return Ok(login_failed_response(cookies));
    }

    let jti = uuid::Uuid::new_v4();

    let jwt_token = auth_tokens::encode_token(
        jwt_secret.jwt_secret(),
        user.id,
        user.role.into(),
        jti,
        user.display_name,
        jwt_duration.jwt_duration(),
    );
    let refresh_token_repository = RefreshTokenRepo::new(&conn);
    let refresh_token = match create_refresh_token(user.id, jti, refresh_token_repository).await {
        Ok(value) => value,
        Err(err) => {
            return Ok(auth_error_response(err, cookies));
        }
    };
    let mut refresh_cookie = Cookie::new("refresh_token", refresh_token.to_string());
    refresh_cookie.set_path("/");
    refresh_cookie.set_http_only(true);
    Ok(auth_ok_response(
        jwt_token,
        refresh_token,
        refresh_cookie,
        cookies,
    ))
}

pub async fn admin_login(
    ValidatedJson(request): ValidatedJson<LoginRequest>,
    cookies: Cookies,
    Extension(repo): Extension<DynRepo>,
    Extension(jwt_secret): Extension<DynJwtSecret>,
    Extension(jwt_duration): Extension<DynJwtDuration>,
) -> Result<impl IntoResponse, AppError> {
    let conn = repo.get_async_conn();
    let user_repository = UserRepo::new(&conn);
    let user = match user_repository.find_one_by_email(request.email).await? {
        None => {
            return Ok(login_failed_response(cookies));
        }
        Some(value) => value,
    };
    if user.role != UserRole::Admin.into() {
        return Ok(auth_unauthorized_response(
            "You are not authorized to login here",
            cookies,
        ));
    }

    let password_match = passwords::verify(user.password.as_str(), request.password.as_bytes());

    if !password_match {
        return Ok(login_failed_response(cookies));
    }

    let jti = uuid::Uuid::new_v4();

    let jwt_token = auth_tokens::encode_admin_token(
        jwt_secret.jwt_secret(),
        user.id,
        user.role.into(),
        jti,
        user.display_name,
        jwt_duration.jwt_duration(),
    );
    let refresh_token_repository = RefreshTokenRepo::new(&conn);
    let refresh_token = match create_refresh_token(user.id, jti, refresh_token_repository).await {
        Ok(value) => value,
        Err(err) => {
            return Ok(auth_error_response(err, cookies));
        }
    };
    let mut refresh_cookie = Cookie::new("refresh_token_admin", refresh_token.to_string());
    refresh_cookie.set_path("/");
    refresh_cookie.set_http_only(true);
    Ok(auth_ok_response(
        jwt_token,
        refresh_token,
        refresh_cookie,
        cookies,
    ))
}

pub async fn register(
    ValidatedJson(request): ValidatedJson<RegisterRequest>,
    cookies: Cookies,
    Extension(repo): Extension<DynRepo>,
    Extension(jwt_secret): Extension<DynJwtSecret>,
    Extension(jwt_duration): Extension<DynJwtDuration>,
    Extension(email_sender): Extension<DynEmailSender>,
) -> Result<impl IntoResponse, AppError> {
    let conn = repo.get_async_conn();
    let user_repository = UserRepo::new(&conn);
    if user_repository
        .find_one_by_email(request.email.clone())
        .await?
        .is_some()
    {
        return Ok(simple_error_response(
            String::from("This E-Mail Address is already in use"),
            StatusCode::CONFLICT,
        ));
    }
    if user_repository
        .find_one_by_display_name(request.display_name.clone())
        .await?
        .is_some()
    {
        return Ok(simple_error_response(
            String::from("This Display Name is already in use"),
            StatusCode::CONFLICT,
        ));
    }
    let new_user = NewUser {
        email: request.email.clone(),
        display_name: request.display_name.clone(),
        password: passwords::hash(request.password.as_bytes()),
        role: UserRole::Ghost,
    };

    let user_result = user_repository.insert_one(new_user).await?;

    let jti = uuid::Uuid::new_v4();

    let jwt_token = auth_tokens::encode_token(
        jwt_secret.jwt_secret(),
        user_result.id,
        UserRole::Ghost,
        jti,
        request.display_name.clone(),
        jwt_duration.jwt_duration(),
    );
    let refresh_token_repository = RefreshTokenRepo::new(&conn);
    let refresh_token = create_refresh_token(user_result.id, jti, refresh_token_repository).await?;
    let verify_email_tokens_repository = VerifyEmailTokenRepo::new(&conn);
    let token = create_verify_email_token(
        verify_email_tokens_repository,
        request.email.clone(),
        None,
        user_result.id,
    )
    .await?;
    email_sender.email_sender().send_email_verification_email(
        request.email,
        request.display_name,
        token,
    )?;

    let mut refresh_cookie = Cookie::new("refresh_token", refresh_token.to_string());
    refresh_cookie.set_path("/".to_string());
    refresh_cookie.set_http_only(true);

    Ok(auth_ok_response(
        jwt_token,
        refresh_token,
        refresh_cookie,
        cookies,
    ))
}

pub async fn refresh(
    ValidatedJson(request): ValidatedJson<RefreshRequest>,
    cookies: Cookies,
    Extension(repo): Extension<DynRepo>,
    Extension(jwt_secret): Extension<DynJwtSecret>,
    Extension(jwt_duration): Extension<DynJwtDuration>,
) -> Result<impl IntoResponse, AppError> {
    let refresh_token = cookies.get("refresh_token").map(|c| c.value().to_string());
    let refresh_token_admin = cookies
        .get("refresh_token_admin")
        .map(|c| c.value().to_string());
    let jwt_token = request.token;
    let claims = match decode_token(jwt_secret.jwt_secret(), &jwt_token) {
        Ok(value) => value,
        Err(err) => {
            return Ok(auth_error_response(err, cookies));
        }
    };

    if refresh_token.is_none() && refresh_token_admin.is_none() {
        return Ok(bad_request_response(
            "Authentication: Missing refresh token, 0",
        ));
    }

    let (refresh_token, refresh_cookie) = if claims.is_for_admin_site() {
        if let Some(token) = refresh_token_admin {
            let mut cookie = Cookie::new("refresh_token_admin", token.clone());
            cookie.set_path("/");
            cookie.set_http_only(true);
            (token, cookie)
        } else {
            return Ok(bad_request_response(
                "Authentication: Missing refresh token, 1",
            ));
        }
    } else if let Some(token) = refresh_token {
        let mut cookie = Cookie::new("refresh_token", token.clone());
        cookie.set_path("/");
        cookie.set_http_only(true);
        (token, cookie)
    } else {
        return Ok(bad_request_response(
            "Authentication: Missing refresh token, 2",
        ));
    };

    let id_value: uuid::Uuid = match uuid::Uuid::parse_str(&refresh_token) {
        Ok(value) => value,
        Err(err) => {
            return Ok(bad_request_response(err.to_string()));
        }
    };
    let conn = repo.get_async_conn();
    let refresh_token_repository = RefreshTokenRepo::new(&conn);
    let mut token_data = match refresh_token_repository.find_one(id_value).await? {
        Some(value) => value,
        None => {
            return Ok(auth_unauthorized_response("Invalid Refresh Token", cookies));
        }
    };
    if token_data.invalidated {
        return Ok(bad_request_response("Invalidated Refresh Token"));
    }
    if token_data.used {
        return Ok(bad_request_response("Used Refresh Token"));
    }
    if claims.jti() != token_data.jwt_id || claims.user_id() != token_data.user_id {
        return Ok(bad_request_response("Invalid Auth Token Combination"));
    }
    token_data.used = true;

    refresh_token_repository.use_up(id_value).await?;
    let jti = uuid::Uuid::new_v4();

    let jwt_token = claims
        .new_refreshed(jti, jwt_duration.jwt_duration())
        .to_token(jwt_secret.jwt_secret());
    let refresh_token =
        create_refresh_token(claims.user_id(), jti, refresh_token_repository).await?;
    Ok(auth_ok_response(
        jwt_token,
        refresh_token,
        refresh_cookie,
        cookies,
    ))
}

pub async fn logout(
    cookies: Cookies,
    Extension(repo): Extension<DynRepo>,
) -> Result<impl IntoResponse, AppError> {
    let refresh_token = cookies
        .get("refresh_token")
        .map(|c| c.value().to_string())
        .unwrap_or_default();
    let id_value: uuid::Uuid = match uuid::Uuid::parse_str(&refresh_token) {
        Ok(value) => value,
        Err(err) => {
            return Ok(auth_bad_request_response(err.to_string().as_str(), cookies));
        }
    };
    let conn = repo.get_async_conn();
    let refresh_token_repository = RefreshTokenRepo::new(&conn);
    let _ = match refresh_token_repository.find_one(id_value).await? {
        Some(value) => value,
        None => {
            return Ok(auth_unauthorized_response("Invalid Refresh Token", cookies));
        }
    };
    refresh_token_repository.invalidate(id_value).await?;
    let resp_body = Json(BaseResponse {
        data: Some(()),
        success: Some(true),
        errors: None,
        messages: None,
        pagination: None,
    });

    cookies.remove(Cookie::new("refresh_token", ""));
    Ok((StatusCode::NO_CONTENT, resp_body).into_response())
}

pub async fn logout_admin(
    cookies: Cookies,
    Extension(repo): Extension<DynRepo>,
) -> Result<impl IntoResponse, AppError> {
    let refresh_token = cookies
        .get("refresh_token_admin")
        .map(|c| c.value().to_string())
        .unwrap_or_default();
    let id_value: uuid::Uuid = match uuid::Uuid::parse_str(&refresh_token) {
        Ok(value) => value,
        Err(err) => {
            return Ok(auth_bad_request_response(err.to_string().as_str(), cookies));
        }
    };
    let conn = repo.get_async_conn();
    let refresh_token_repository = RefreshTokenRepo::new(&conn);
    let _ = match refresh_token_repository.find_one(id_value).await? {
        Some(value) => value,
        None => {
            return Ok(auth_unauthorized_response("Invalid Refresh Token", cookies));
        }
    };
    refresh_token_repository.invalidate(id_value).await?;
    let resp_body = Json(BaseResponse {
        data: Some(()),
        success: Some(true),
        errors: None,
        messages: None,
        pagination: None,
    });
    cookies.remove(Cookie::new("refresh_token_admin", ""));
    Ok((StatusCode::NO_CONTENT, resp_body).into_response())
}

pub async fn request_verification_email(
    ClaimsContext { claims }: ClaimsContext,
    ValidatedJson(request): ValidatedJson<RequestVerificationEmailRequest>,
    Extension(repo): Extension<DynRepo>,
    Extension(email_sender): Extension<DynEmailSender>,
) -> Result<impl IntoResponse, AppError> {
    let user_id: i32;
    let old_email: Option<String>;
    let display_name: String;

    let conn = repo.get_async_conn();
    let user_repository = UserRepo::new(&conn);
    let mut email: String = match user_repository.find_one(claims.user_id()).await? {
        Some(user) => {
            user_id = user.id;
            display_name = user.display_name;
            user.email
        }
        None => {
            return Ok(simple_error_response(
                String::from("Invalid User Id in JWT."),
                StatusCode::BAD_REQUEST,
            ));
        }
    };
    if let Some(new_email) = request.email {
        old_email = Some(email);
        email = new_email;
    } else {
        old_email = None;
    }

    let verify_email_tokens_repository = VerifyEmailTokenRepo::new(&conn);
    let token: String = match create_verify_email_token(
        verify_email_tokens_repository,
        email.clone(),
        old_email,
        user_id,
    )
    .await
    {
        Ok(value) => value,
        Err(err) => {
            return Ok(server_error_response(err));
        }
    };
    email_sender
        .email_sender()
        .send_email_verification_email(email, display_name, token)?;

    Ok(simple_ok_response(()))
}

pub async fn verify_email(
    ValidatedJson(request): ValidatedJson<VerifyEmailRequest>,
    Extension(repo): Extension<DynRepo>,
) -> Result<Response<BoxBody>, AppError> {
    let conn = repo.get_async_conn();
    let verify_email_tokens_repository = VerifyEmailTokenRepo::new(&conn);
    let token_data = match verify_email_tokens_repository
        .find_one_by_token(request.token)
        .await?
    {
        Some(value) => value,
        None => {
            return Ok(bad_request_response("Invalid Token"));
        }
    };
    if token_data.used {
        return Ok(simple_error_response(
            String::from("This token is already used"),
            StatusCode::CONFLICT,
        ));
    } else if token_data.invalidated {
        return Ok(simple_error_response(
            String::from("This token is invalidated"),
            StatusCode::CONFLICT,
        ));
    } else if token_data.expires_at <= Utc::now().naive_utc() {
        return Ok(simple_error_response(
            String::from("This token is expired"),
            StatusCode::CONFLICT,
        ));
    }

    let user_repository = UserRepo::new(&conn);
    let user = match user_repository.find_one(token_data.user_id).await? {
        Some(value) => value,
        None => {
            return Ok(bad_request_response(
                "Invalid Token Data, couldn't find User",
            ));
        }
    };
    let mut updated_user = UpdateUser {
        display_name: None,
        email: None,
        password: None,
        role: None,
        updated_at: Some(Some(Utc::now().naive_utc())),
    };
    if user.role == UserRole::Ghost.into() {
        updated_user.role = Some(UserRole::User);
    }
    user_repository.update_one(user.id, updated_user).await?;
    let updated_token = UpdateVerifyEmailToken {
        invalidated: None,
        used: Some(true),
    };
    verify_email_tokens_repository
        .update_one(token_data.id, updated_token)
        .await?;
    Ok(simple_ok_response(()))
}

pub async fn request_reset_password_email(
    ValidatedJson(request): ValidatedJson<RequestResetPasswordEmailRequest>,
    Extension(repo): Extension<DynRepo>,
    Extension(email_sender): Extension<DynEmailSender>,
) -> Result<impl IntoResponse, AppError> {
    let conn = repo.get_async_conn();
    let user_repository = UserRepo::new(&conn);
    let user = match user_repository
        .find_one_by_email(request.email.clone())
        .await?
    {
        Some(value) => value,
        None => {
            return Ok(bad_request_response("Couldn't find User"));
        }
    };

    let change_password_tokens_repository = ChangePasswordTokenRepo::new(&conn);
    let token: String =
        match create_reset_password_token(change_password_tokens_repository, user.id).await {
            Ok(value) => value,
            Err(err) => {
                return Ok(server_error_response(err));
            }
        };
    email_sender.email_sender().send_reset_password_email(
        request.email,
        user.display_name,
        token,
    )?;

    Ok(simple_ok_response(()))
}

pub async fn reset_password(
    ValidatedJson(request): ValidatedJson<ResetPasswordRequest>,
    Extension(repo): Extension<DynRepo>,
) -> Result<Response<BoxBody>, AppError> {
    let conn = repo.get_async_conn();
    let change_password_tokens_repository = ChangePasswordTokenRepo::new(&conn);
    let token_data = match change_password_tokens_repository
        .find_one_by_token(request.token)
        .await?
    {
        Some(value) => value,
        None => {
            return Ok(bad_request_response("Invalid Token"));
        }
    };
    if token_data.used {
        return Ok(simple_error_response(
            String::from("This token is already used"),
            StatusCode::CONFLICT,
        ));
    } else if token_data.invalidated {
        return Ok(simple_error_response(
            String::from("This token is invalidated"),
            StatusCode::CONFLICT,
        ));
    } else if token_data.expires_at <= Utc::now().naive_utc() {
        return Ok(simple_error_response(
            String::from("This token is expired"),
            StatusCode::CONFLICT,
        ));
    }
    let user_repository = UserRepo::new(&conn);
    let user = match user_repository.find_one(token_data.user_id).await? {
        Some(value) => value,
        None => {
            return Ok(bad_request_response(
                "Invalid Token Data, couldn't find User",
            ));
        }
    };
    let new_password_hash = passwords::hash(request.new_password.as_bytes());
    let updated_user = UpdateUser {
        display_name: None,
        email: None,
        password: Some(new_password_hash),
        role: None,
        updated_at: Some(Some(Utc::now().naive_utc())),
    };

    user_repository.update_one(user.id, updated_user).await?;
    let updated_token = UpdateChangePasswordToken {
        invalidated: None,
        used: Some(true),
    };
    change_password_tokens_repository
        .update_one(token_data.id, updated_token)
        .await?;
    Ok(simple_ok_response(()))
}

pub async fn get_profile(
    ClaimsContext { claims }: ClaimsContext,
    Extension(repo): Extension<DynRepo>,
) -> Result<impl IntoResponse, AppError> {
    let conn = repo.get_async_conn();
    let user_repository = UserRepo::new(&conn);
    let user: User = match user_repository.find_one(claims.user_id()).await? {
        Some(value) => value.into(),
        None => {
            return Ok(not_found_response("User"));
        }
    };
    Ok(simple_ok_response(user))
}

async fn create_verify_email_token(
    verify_email_tokens_repository: backend_repo_pg::verify_email_tokens::VerifyEmailTokenRepo<'_>,
    email: String,
    old_email: Option<String>,
    user_id: i32,
) -> Result<String, PgRepoError> {
    let token = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .map(char::from)
        .take(75)
        .collect::<String>();

    let new_verify_email_token = NewVerifyEmailToken {
        email,
        expires_at: (Utc::now() + Duration::days(30)).naive_utc(),
        old_email,
        token,
        user_id,
    };
    let inserted_token = verify_email_tokens_repository
        .insert_one(new_verify_email_token)
        .await?;

    Ok(inserted_token.token)
}

async fn create_reset_password_token(
    change_password_tokens_repository: backend_repo_pg::change_password_tokens::ChangePasswordTokenRepo<'_>,
    user_id: i32,
) -> Result<String, PgRepoError> {
    let token = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .map(char::from)
        .take(75)
        .collect::<String>();

    let new_change_password_token = NewChangePasswordToken {
        expires_at: (Utc::now() + Duration::days(30)).naive_utc(),
        token,
        user_id,
    };
    let inserted_token = change_password_tokens_repository
        .insert_one(new_change_password_token)
        .await?;

    Ok(inserted_token.token)
}
