use crate::cookies::CookieBuilder;
use crate::util::{
    auth_bad_request_response, auth_error_response, auth_ok_response, auth_unauthorized_response,
    bad_request_response, create_refresh_token, login_failed_response, not_found_response,
    server_error_response,
};
use crate::{app::AppState, auth_tokens::Claims};
use crate::{auth_tokens, util::simple_error_response};
use crate::{auth_tokens::decode_token, util::simple_ok_response};
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
use rand::{distributions::Alphanumeric, Rng};
use warp::hyper::header;
use warp::{http::StatusCode, Reply};

pub async fn login(
    request: LoginRequest,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let user_repository = UserRepo::new(state.repo.clone());
    let user = match user_repository.find_one_by_email(request.email).await {
        Err(err) => {
            return Ok(auth_error_response(err));
        }
        Ok(value_opt) => match value_opt {
            None => {
                return Ok(login_failed_response());
            }
            Some(value) => value,
        },
    };

    let password_match = passwords::verify(user.password.as_str(), request.password.as_bytes());

    if password_match == false {
        return Ok(login_failed_response());
    }

    let jti = uuid::Uuid::new_v4();

    let jwt_token = auth_tokens::encode_token(
        state.jwt_secret.as_str(),
        user.id,
        user.role,
        jti,
        user.display_name,
        state.jwt_duration,
    );
    let refresh_token_repository = RefreshTokenRepo::new(state.repo.clone());
    let refresh_token = match create_refresh_token(user.id, jti, refresh_token_repository).await {
        Ok(value) => value,
        Err(err) => {
            return Ok(auth_error_response(err));
        }
    };
    let refresh_cookie_builder = CookieBuilder::new()
        .with_name("refresh_token".into())
        .with_path("/".into())
        .with_http_only();
    Ok(auth_ok_response(
        jwt_token,
        refresh_token,
        refresh_cookie_builder,
    ))
}

pub async fn admin_login(
    request: LoginRequest,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let user_repository = UserRepo::new(state.repo.clone());
    let user = match user_repository.find_one_by_email(request.email).await {
        Err(err) => {
            return Ok(auth_error_response(err));
        }
        Ok(value_opt) => match value_opt {
            None => {
                return Ok(login_failed_response());
            }
            Some(value) => value,
        },
    };
    if user.role != UserRole::Admin {
        return Ok(auth_unauthorized_response(
            "You are not authorized to login here",
        ));
    }

    let password_match = passwords::verify(user.password.as_str(), request.password.as_bytes());

    if password_match == false {
        return Ok(login_failed_response());
    }

    let jti = uuid::Uuid::new_v4();

    let jwt_token = auth_tokens::encode_admin_token(
        state.jwt_secret.as_str(),
        user.id,
        user.role,
        jti,
        user.display_name,
        state.jwt_duration,
    );
    let refresh_token_repository = RefreshTokenRepo::new(state.repo.clone());
    let refresh_token = match create_refresh_token(user.id, jti, refresh_token_repository).await {
        Ok(value) => value,
        Err(err) => {
            return Ok(auth_error_response(err));
        }
    };
    let refresh_cookie_builder = CookieBuilder::new()
        .with_name("refresh_token_admin".into())
        .with_path("/".into())
        .with_http_only();
    Ok(auth_ok_response(
        jwt_token,
        refresh_token,
        refresh_cookie_builder,
    ))
}

pub async fn register(
    request: RegisterRequest,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let user_repository = UserRepo::new(state.repo.clone());
    match user_repository
        .find_one_by_email(request.email.clone())
        .await
    {
        Err(err) => {
            return Ok(server_error_response(err));
        }
        Ok(Some(_)) => {
            return Ok(simple_error_response(
                String::from("This E-Mail Address is already in use"),
                StatusCode::CONFLICT,
            ));
        }
        Ok(None) => {}
    }
    match user_repository
        .find_one_by_display_name(request.display_name.clone())
        .await
    {
        Err(err) => {
            return Ok(server_error_response(err));
        }
        Ok(Some(_)) => {
            return Ok(simple_error_response(
                String::from("This Display Name is already in use"),
                StatusCode::CONFLICT,
            ));
        }
        Ok(None) => {}
    }
    let new_user = NewUser {
        email: request.email.clone(),
        display_name: request.display_name.clone(),
        password: passwords::hash(request.password.as_bytes()),
        role: UserRole::Ghost,
    };

    let user_result = match user_repository.insert_one(new_user).await {
        Err(err) => {
            return Ok(auth_error_response(err));
        }
        Ok(value) => value,
    };

    let jti = uuid::Uuid::new_v4();

    let jwt_token = auth_tokens::encode_token(
        state.jwt_secret.as_str(),
        user_result.id,
        UserRole::Ghost,
        jti.clone(),
        request.display_name.clone(),
        state.jwt_duration,
    );
    let refresh_token_repository = RefreshTokenRepo::new(state.repo.clone());
    let refresh_token =
        match create_refresh_token(user_result.id, jti, refresh_token_repository).await {
            Ok(value) => value,
            Err(err) => {
                return Ok(auth_error_response(err));
            }
        };
    let verify_email_tokens_repository = VerifyEmailTokenRepo::new(state.repo.clone());
    let token = match create_verify_email_token(
        verify_email_tokens_repository,
        request.email.clone(),
        None,
        user_result.id,
    )
    .await
    {
        Ok(value) => value,
        Err(err) => {
            return Ok(auth_error_response(err));
        }
    };
    state
        .email_sender
        .send_email_verification_email(request.email, request.display_name, token)
        .await?;

    let refresh_cookie_builder = CookieBuilder::new()
        .with_name("refresh_token".into())
        .with_path("/".into())
        .with_http_only();
    Ok(auth_ok_response(
        jwt_token,
        refresh_token,
        refresh_cookie_builder,
    ))
}

pub async fn refresh(
    request: RefreshRequest,
    refresh_token: Option<String>,
    refresh_token_admin: Option<String>,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let jwt_token = request.token;
    let claims = match decode_token(&state.jwt_secret, &jwt_token) {
        Ok(value) => value,
        Err(err) => {
            return Ok(auth_error_response(err));
        }
    };

    if refresh_token.is_none() && refresh_token_admin.is_none() {
        return Ok(bad_request_response(
            "Authentication: Missing refresh token, 0",
        ));
    }

    let (refresh_token, refresh_cookie_builder) = if claims.is_for_admin_site() {
        if let Some(token) = refresh_token_admin {
            (
                token,
                CookieBuilder::new()
                    .with_name("refresh_token_admin".into())
                    .with_path("/".into())
                    .with_http_only(),
            )
        } else {
            return Ok(bad_request_response(
                "Authentication: Missing refresh token, 1",
            ));
        }
    } else {
        if let Some(token) = refresh_token {
            (
                token,
                CookieBuilder::new()
                    .with_name("refresh_token".into())
                    .with_path("/".into())
                    .with_http_only(),
            )
        } else {
            return Ok(bad_request_response(
                "Authentication: Missing refresh token, 2",
            ));
        }
    };

    println!("{}", refresh_token);

    let id_value: uuid::Uuid = match uuid::Uuid::parse_str(refresh_token.as_str()) {
        Ok(value) => value,
        Err(err) => {
            return Ok(bad_request_response(err.to_string().as_str()));
        }
    };
    let refresh_token_repository = RefreshTokenRepo::new(state.repo.clone());
    let mut token_data = match refresh_token_repository.find_one(id_value.clone()).await {
        Ok(opt) => match opt {
            Some(value) => value,
            None => {
                return Ok(auth_unauthorized_response("Invalid Refresh Token"));
            }
        },
        Err(err) => {
            return Ok(auth_error_response(err));
        }
    };
    if token_data.invalidated == true {
        return Ok(bad_request_response("Invalidated Refresh Token"));
    }
    if token_data.used == true {
        return Ok(bad_request_response("Used Refresh Token"));
    }
    if claims.jti() != token_data.jwt_id || claims.user_id() != token_data.user_id {
        return Ok(bad_request_response("Invalid Auth Token Combination"));
    }
    token_data.used = true;

    match refresh_token_repository.use_up(id_value).await {
        Ok(_) => {}
        Err(err) => {
            return Ok(auth_error_response(err));
        }
    }
    let jti = uuid::Uuid::new_v4();

    let jwt_token = claims
        .new_refreshed(jti, state.jwt_duration)
        .to_token(state.jwt_secret.as_str());
    let refresh_token =
        match create_refresh_token(claims.user_id(), jti, refresh_token_repository).await {
            Ok(value) => value,
            Err(err) => {
                return Ok(auth_error_response(err));
            }
        };
    Ok(auth_ok_response(
        jwt_token,
        refresh_token,
        refresh_cookie_builder,
    ))
}

pub async fn logout(
    refresh_token: String,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let id_value: uuid::Uuid = match uuid::Uuid::parse_str(refresh_token.as_str()) {
        Ok(value) => value,
        Err(err) => {
            return Ok(auth_bad_request_response(err.to_string().as_str()));
        }
    };
    let refresh_token_repository = RefreshTokenRepo::new(state.repo.clone());
    let _ = match refresh_token_repository.find_one(id_value.clone()).await {
        Ok(value_opt) => match value_opt {
            Some(value) => value,
            None => {
                return Ok(auth_unauthorized_response("Invalid Refresh Token"));
            }
        },
        Err(err) => {
            return Ok(auth_error_response(err));
        }
    };
    match refresh_token_repository.invalidate(id_value).await {
        Ok(_) => {}
        Err(err) => {
            return Ok(auth_error_response(err));
        }
    }
    let resp_body = warp::reply::json(&BaseResponse {
        data: Some(()),
        success: Some(true),
        errors: None,
        messages: None,
        pagination: None,
    });
    let resp_with_status = warp::reply::with_status(resp_body, StatusCode::NO_CONTENT);
    let resp_with_header =
        warp::reply::with_header(resp_with_status, header::SET_COOKIE, "refresh_token=");
    return Ok(resp_with_header.into_response());
}

pub async fn logout_admin(
    refresh_token: String,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let id_value: uuid::Uuid = match uuid::Uuid::parse_str(refresh_token.as_str()) {
        Ok(value) => value,
        Err(err) => {
            return Ok(auth_bad_request_response(err.to_string().as_str()));
        }
    };
    let refresh_token_repository = RefreshTokenRepo::new(state.repo.clone());
    let _ = match refresh_token_repository.find_one(id_value.clone()).await {
        Ok(value_opt) => match value_opt {
            Some(value) => value,
            None => {
                return Ok(auth_unauthorized_response("Invalid Refresh Token"));
            }
        },
        Err(err) => {
            return Ok(auth_error_response(err));
        }
    };
    match refresh_token_repository.invalidate(id_value).await {
        Ok(_) => {}
        Err(err) => {
            return Ok(auth_error_response(err));
        }
    }
    let resp_body = warp::reply::json(&BaseResponse {
        data: Some(()),
        success: Some(true),
        errors: None,
        messages: None,
        pagination: None,
    });
    let resp_with_status = warp::reply::with_status(resp_body, StatusCode::NO_CONTENT);
    let resp_with_header =
        warp::reply::with_header(resp_with_status, header::SET_COOKIE, "refresh_token_admin=");
    return Ok(resp_with_header.into_response());
}

pub async fn request_verification_email(
    claims: Claims,
    request: RequestVerificationEmailRequest,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let user_id: i32;
    let old_email: Option<String>;
    let display_name: String;

    let user_repository = UserRepo::new(state.repo.clone());
    let mut email: String = match user_repository.find_one(claims.user_id()).await {
        Ok(Some(user)) => match user.email {
            Some(value) => {
                user_id = user.id;
                display_name = user.display_name;
                value
            }
            None => {
                return Ok(simple_error_response(
                    String::from("This should never happen, but couldn't find email."),
                    StatusCode::INTERNAL_SERVER_ERROR,
                ));
            }
        },
        Ok(None) => {
            return Ok(simple_error_response(
                String::from("Invalid User Id in JWT."),
                StatusCode::BAD_REQUEST,
            ));
        }
        Err(err) => {
            return Ok(server_error_response(err));
        }
    };
    if let Some(new_email) = request.email {
        old_email = Some(email);
        email = new_email;
    } else {
        old_email = None;
    }

    let verify_email_tokens_repository = VerifyEmailTokenRepo::new(state.repo.clone());
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
    state
        .email_sender
        .send_email_verification_email(email, display_name, token)
        .await?;

    return Ok(simple_ok_response(()));
}

pub async fn verify_email(
    request: VerifyEmailRequest,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let verify_email_tokens_repository = VerifyEmailTokenRepo::new(state.repo.clone());
    let token_data = match verify_email_tokens_repository
        .find_one_by_token(request.token)
        .await
    {
        Ok(Some(value)) => value,
        Ok(None) => {
            return Ok(bad_request_response("Invalid Token"));
        }
        Err(err) => {
            return Ok(server_error_response(err));
        }
    };
    if token_data.used == true {
        return Ok(simple_error_response(
            String::from("This token is already used"),
            StatusCode::CONFLICT,
        ));
    } else if token_data.invalidated == true {
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
    let user_repository = UserRepo::new(state.repo.clone());
    let user = match user_repository.find_one(token_data.user_id).await {
        Ok(Some(value)) => value,
        Ok(None) => {
            return Ok(bad_request_response(
                "Invalid Token Data, couldn't find User",
            ));
        }
        Err(err) => {
            return Ok(server_error_response(err));
        }
    };
    let mut updated_user = UpdateUser {
        display_name: None,
        email: None,
        password: None,
        role: None,
        updated_at: Some(Some(Utc::now().naive_utc())),
    };
    if user.role == UserRole::Ghost {
        updated_user.role = Some(UserRole::User);
    }
    match user_repository.update_one(user.id, updated_user).await {
        Ok(_) => {}
        Err(err) => {
            return Ok(server_error_response(err));
        }
    };
    let updated_token = UpdateVerifyEmailToken {
        invalidated: None,
        used: Some(true),
    };
    match verify_email_tokens_repository
        .update_one(token_data.id, updated_token)
        .await
    {
        Ok(_) => {}
        Err(err) => {
            return Ok(server_error_response(err));
        }
    }
    return Ok(simple_ok_response(()));
}

pub async fn request_reset_password_email(
    request: RequestResetPasswordEmailRequest,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let user_repository = UserRepo::new(state.repo.clone());
    let user = match user_repository
        .find_one_by_email(request.email.clone())
        .await
    {
        Ok(Some(value)) => value,
        Ok(None) => {
            return Ok(bad_request_response("Couldn't find User"));
        }
        Err(err) => {
            return Ok(server_error_response(err));
        }
    };

    let change_password_tokens_repository = ChangePasswordTokenRepo::new(state.repo.clone());
    let token: String =
        match create_reset_password_token(change_password_tokens_repository, user.id).await {
            Ok(value) => value,
            Err(err) => {
                return Ok(server_error_response(err));
            }
        };
    state
        .email_sender
        .send_reset_password_email(request.email, user.display_name, token)
        .await?;

    return Ok(simple_ok_response(()));
}

pub async fn reset_password(
    request: ResetPasswordRequest,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let change_password_tokens_repository = ChangePasswordTokenRepo::new(state.repo.clone());
    let token_data = match change_password_tokens_repository
        .find_one_by_token(request.token)
        .await
    {
        Ok(Some(value)) => value,
        Ok(None) => {
            return Ok(bad_request_response("Invalid Token"));
        }
        Err(err) => {
            return Ok(server_error_response(err));
        }
    };
    if token_data.used == true {
        return Ok(simple_error_response(
            String::from("This token is already used"),
            StatusCode::CONFLICT,
        ));
    } else if token_data.invalidated == true {
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
    let user_repository = UserRepo::new(state.repo.clone());
    let user = match user_repository.find_one(token_data.user_id).await {
        Ok(Some(value)) => value,
        Ok(None) => {
            return Ok(bad_request_response(
                "Invalid Token Data, couldn't find User",
            ));
        }
        Err(err) => {
            return Ok(server_error_response(err));
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

    match user_repository.update_one(user.id, updated_user).await {
        Ok(_) => {}
        Err(err) => {
            return Ok(server_error_response(err));
        }
    };
    let updated_token = UpdateChangePasswordToken {
        invalidated: None,
        used: Some(true),
    };
    match change_password_tokens_repository
        .update_one(token_data.id, updated_token)
        .await
    {
        Ok(_) => {}
        Err(err) => {
            return Ok(server_error_response(err));
        }
    }
    return Ok(simple_ok_response(()));
}

pub async fn get_profile(
    claims: Claims,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let user_repository = UserRepo::new(state.repo.clone());
    let user = match user_repository.find_one(claims.user_id()).await {
        Ok(Some(value)) => value,
        Ok(None) => {
            return Ok(not_found_response("User"));
        }
        Err(err) => {
            return Ok(server_error_response(err));
        }
    };
    return Ok(simple_ok_response(user));
}

async fn create_verify_email_token(
    verify_email_tokens_repository: backend_repo_pg::verify_email_tokens::VerifyEmailTokenRepo,
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

    return Ok(inserted_token.token);
}

async fn create_reset_password_token(
    change_password_tokens_repository: backend_repo_pg::change_password_tokens::ChangePasswordTokenRepo,
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

    return Ok(inserted_token.token);
}
