use crate::app::AppState;
use crate::auth_tokens;
use crate::auth_tokens::decode_token;
use crate::util::{
    auth_bad_request_response, auth_error_response, auth_ok_response, auth_unauthorized_response,
    create_refresh_token, login_failed_response,
};
use backend_repo_pg::models::responses::BaseResponse;
use backend_repo_pg::passwords;
use backend_repo_pg::{
    extra::UserRole,
    insertables::NewUser,
    models::requests::{LoginRequest, RefreshRequest, RegisterRequest},
};
use warp::http::StatusCode;
use warp::hyper::header;

pub async fn login(
    request: LoginRequest,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let user = match state
        .repository
        .user_repository
        .find_one_by_email(request.email)
        .await
    {
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

    let jwt_token = match user.role {
        UserRole::Admin => auth_tokens::encode_admin_token(
            state.jwt_secret.as_str(),
            user.id,
            user.role,
            jti,
            user.display_name,
            state.jwt_duration,
        ),
        _ => auth_tokens::encode_token(
            state.jwt_secret.as_str(),
            user.id,
            user.role,
            jti,
            user.display_name,
            state.jwt_duration,
        ),
    };
    let refresh_token =
        match create_refresh_token(user.id, jti, state.repository.refresh_token_repository).await {
            Ok(value) => value,
            Err(err) => {
                return Ok(auth_error_response(err));
            }
        };
    Ok(auth_ok_response(
        jwt_token,
        refresh_token,
        state.refresh_cookie_builder,
    ))
}

pub async fn admin_login(
    request: LoginRequest,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let user = match state
        .repository
        .user_repository
        .find_one_by_email(request.email)
        .await
    {
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
    let refresh_token =
        match create_refresh_token(user.id, jti, state.repository.refresh_token_repository).await {
            Ok(value) => value,
            Err(err) => {
                return Ok(auth_error_response(err));
            }
        };
    Ok(auth_ok_response(
        jwt_token,
        refresh_token,
        state.refresh_cookie_builder,
    ))
}

pub async fn register(
    request: RegisterRequest,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let new_user = NewUser {
        email: request.email,
        display_name: request.display_name.clone(),
        password: passwords::hash(request.password.as_bytes()),
        role: UserRole::Ghost,
    };

    let user_result = match state.repository.user_repository.insert_one(new_user).await {
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
        request.display_name,
        state.jwt_duration,
    );
    let refresh_token = match create_refresh_token(
        user_result.id,
        jti,
        state.repository.refresh_token_repository,
    )
    .await
    {
        Ok(value) => value,
        Err(err) => {
            return Ok(auth_error_response(err));
        }
    };
    Ok(auth_ok_response(
        jwt_token,
        refresh_token,
        state.refresh_cookie_builder,
    ))
}

pub async fn refresh(
    request: RefreshRequest,
    refresh_token: String,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let id_value: uuid::Uuid = match uuid::Uuid::parse_str(refresh_token.as_str()) {
        Ok(value) => value,
        Err(err) => {
            return Ok(auth_bad_request_response(err.to_string().as_str()));
        }
    };
    let mut token_data = match state
        .repository
        .refresh_token_repository
        .find_one(id_value.clone())
        .await
    {
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
        return Ok(auth_bad_request_response("Invalidated Refresh Token"));
    }
    if token_data.used == true {
        return Ok(auth_bad_request_response("Used Refresh Token"));
    }
    let jwt_token = request.token;
    let claims = match decode_token(&state.jwt_secret, &jwt_token) {
        Ok(value) => value,
        Err(err) => {
            return Ok(auth_error_response(err));
        }
    };
    if claims.jti() != token_data.jwt_id || claims.user_id() != token_data.user_id {
        return Ok(auth_bad_request_response("Invalid Auth Token Combination"));
    }
    token_data.used = true;

    match state
        .repository
        .refresh_token_repository
        .use_up(id_value)
        .await
    {
        Ok(_) => {}
        Err(err) => {
            return Ok(auth_error_response(err));
        }
    }
    let jti = uuid::Uuid::new_v4();

    let jwt_token = claims
        .new_refreshed(jti, state.jwt_duration)
        .to_token(state.jwt_secret.as_str());
    let refresh_token = match create_refresh_token(
        claims.user_id(),
        jti,
        state.repository.refresh_token_repository,
    )
    .await
    {
        Ok(value) => value,
        Err(err) => {
            return Ok(auth_error_response(err));
        }
    };
    Ok(auth_ok_response(
        jwt_token,
        refresh_token,
        state.refresh_cookie_builder,
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
    let _ = match state
        .repository
        .refresh_token_repository
        .find_one(id_value.clone())
        .await
    {
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
    match state
        .repository
        .refresh_token_repository
        .invalidate(id_value)
        .await
    {
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
    return Ok(resp_with_header);
}
