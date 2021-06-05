use std::net::SocketAddr;

use crate::app::AppState;
use crate::util::bad_request_response;
use crate::{
    auth_tokens,
    util::{
        not_found_response, server_error_response, simple_created_response, simple_ok_response,
    },
};
use auth_tokens::Claims;
use backend_repo_pg::{
    identification_cookies::IdentificationCookieRepo, models::requests::CreatePageViewRequest,
    page_views::PageViewRepo,
};
use backend_repo_pg::{
    insertables::{NewIdentificationCookie, NewPageView},
    models::queries::GetPageViewsQuery,
};
use chrono::{Duration, Utc};
use rand::{distributions::Alphanumeric, Rng};
use sha2::{Digest, Sha512};
use urlencoding::decode;
use warp::Reply;

pub async fn get(
    url: String,
    query: GetPageViewsQuery,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(state
        .repo
        .transaction(|conn| {
            let url = match decode(&url) {
                Err(err) => {
                    return Ok(bad_request_response(&err.to_string()));
                }
                Ok(value_opt) => value_opt,
            };
            let pages_views_repository = PageViewRepo::new(&conn);
            let page_views_result = match pages_views_repository.count_by_url(url) {
                Err(err) => {
                    return Ok(server_error_response(err));
                }
                Ok(val) => val,
            };
            Ok(simple_ok_response(page_views_result))
        })
        .await?)
}

pub async fn create(
    user_agent: Option<String>,
    id_cookie: Option<String>,
    addr: Option<SocketAddr>,
    claims: Option<Claims>,
    request: CreatePageViewRequest,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(state
        .repo
        .transaction(|conn| {
            let mut return_cookie = None;

            let identification_cookies_repository = IdentificationCookieRepo::new(&conn);

            let id_hash;

            let id_cookie_result = if let Some(token) = id_cookie {
                match identification_cookies_repository.find_one_by_token(token) {
                    Err(err) => {
                        return Ok(server_error_response(err));
                    }
                    Ok(value_opt) => value_opt,
                }
            } else {
                None
            };

            if let Some(id_cookie) = id_cookie_result {
                id_hash = id_cookie.id_hash;
            } else {
                let mut hasher = Sha512::new();
                let hash_string = format!(
                    "{}{}{}",
                    request.latitude.unwrap_or(200.),
                    request.longitude.unwrap_or(200.),
                    addr.map(|a| a.ip().to_string()).unwrap_or(String::from(""))
                );
                hasher.update(hash_string);

                let hash_result = hasher.finalize();
                let new_id_hash = format!("{:x}", hash_result);

                let existing_cookie =
                    match identification_cookies_repository.find_one_by_hash(new_id_hash.clone()) {
                        Err(err) => {
                            return Ok(server_error_response(err));
                        }
                        Ok(value) => value,
                    };

                return_cookie = if let Some(cookie) = existing_cookie {
                    id_hash = new_id_hash;
                    Some(cookie.token)
                } else {
                    let token = rand::thread_rng()
                        .sample_iter(&Alphanumeric)
                        .map(char::from)
                        .take(35)
                        .collect::<String>();

                    let new_cookie = NewIdentificationCookie {
                        token: token.clone(),
                        id_hash: new_id_hash.clone(),
                        expires_at: (Utc::now() + Duration::days(30 * 6)).naive_utc(),
                    };

                    match identification_cookies_repository.insert_one(new_cookie) {
                        Err(err) => {
                            return Ok(server_error_response(err));
                        }
                        Ok(value) => value,
                    };

                    id_hash = new_id_hash;

                    Some(token)
                }
            }

            let new_view = NewPageView {
                id_hash,
                page_url: request.page_url,
                registered: claims.is_some(),
                user_agent,
                latitude: request.latitude,
                longitude: request.longitude,
                country_code: request.country_code,
            };
            let pages_views_repository = PageViewRepo::new(&conn);
            let view_insert_result = match pages_views_repository.insert_one(new_view) {
                Err(err) => {
                    return Ok(server_error_response(err));
                }
                Ok(value) => value,
            };

            let resp = simple_created_response(view_insert_result);

            if let Some(token) = return_cookie {
                Ok(state
                    .id_cookie_builder
                    .cookie_with_value_and_expires_days(resp, token, 6 * 30)
                    .unwrap()
                    .into_response())
            } else {
                Ok(resp)
            }
        })
        .await?)
}
