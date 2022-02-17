use std::net::SocketAddr;

use crate::errors::{AppError, GeolocError};
use crate::extractors::{OptClaimsContext, ValidatedJson, ValidatedQuery};
use crate::util::bad_request_response;
use crate::util::{server_error_response, simple_created_response, simple_ok_response};
use axum::extract::{ConnectInfo, Extension, Path, TypedHeader};
use axum::response::IntoResponse;
use backend_repo_pg::models::responses::GeolocationDbResponse;
use backend_repo_pg::pg_util::{pg_transaction, DynRepo};
use backend_repo_pg::{
    identification_cookies::IdentificationCookieRepo, models::requests::CreatePageViewRequest,
    page_views::PageViewRepo,
};
use backend_repo_pg::{
    insertables::{NewIdentificationCookie, NewPageView},
    models::queries::GetPageViewsQuery,
};
use headers::UserAgent;
use hyper::body::HttpBody;
use hyper::client::connect::dns::GaiResolver;
use hyper::client::HttpConnector;
use hyper::Response;
use hyper::{Body, Client};
use hyper_tls::HttpsConnector;
use rand::{distributions::Alphanumeric, Rng};
use sha2::{Digest, Sha512};
use time::{Duration, OffsetDateTime};
use tokio::task::block_in_place;
use tower_cookies::{Cookie, Cookies};
use urlencoding::decode;

pub async fn get(
    Path(url): Path<String>,
    ValidatedQuery(_query): ValidatedQuery<GetPageViewsQuery>,
    Extension(repo): Extension<DynRepo>,
) -> Result<impl IntoResponse, AppError> {
    block_in_place(|| {
        let conn = repo.get_conn()?;
        let url = match decode(&url) {
            Err(err) => {
                return Ok(bad_request_response(err.to_string()));
            }
            Ok(value_opt) => value_opt,
        };
        let pages_views_repository = PageViewRepo::new(&conn);
        let page_views_result = match pages_views_repository.count_by_url(&url) {
            Err(err) => {
                return Ok(server_error_response(err));
            }
            Ok(val) => val,
        };
        Ok(simple_ok_response(page_views_result))
    })
}

pub async fn create(
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    cookies: Cookies,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    OptClaimsContext { claims }: OptClaimsContext,
    ValidatedJson(request): ValidatedJson<CreatePageViewRequest>,
    Extension(repo): Extension<DynRepo>,
) -> Result<impl IntoResponse, AppError> {
    let id_cookie = cookies.get("identifier").map(|c| c.value().to_string());
    let user_agent = Some(user_agent.to_string());
    let https = hyper_tls::HttpsConnector::new();
    let geoloc_url = format!("https://geolocation-db.com/json/{}", addr.ip());
    println!("{}", geoloc_url);
    let geoloc_uri = geoloc_url.parse().map_err(GeolocError::new)?;

    let client: Client<HttpsConnector<HttpConnector<GaiResolver>>, Body> =
        Client::builder().build(https);
    let resp: Response<Body> = client.get(geoloc_uri).await.map_err(GeolocError::from)?;

    let body: Vec<u8> = match resp.into_body().data().await {
        Some(Ok(v)) => v.to_vec(),
        _ => {
            return Ok(server_error_response(GeolocError::new(
                "Failed to read Geoloc Response",
            )));
        }
    };
    let geoloc: GeolocationDbResponse = serde_json::from_slice(&body).map_err(GeolocError::from)?;

    let latitude: Option<f64> = geoloc.latitude.parse().ok();
    let longitude: Option<f64> = geoloc.latitude.parse().ok();
    let country_code: Option<String> = if geoloc.country_code.is_empty() {
        None
    } else {
        Some(geoloc.country_code)
    };

    Ok(pg_transaction(repo, |conn| {
        let mut return_cookie = None;

        let identification_cookies_repository = IdentificationCookieRepo::new(conn);

        let id_hash;

        let id_cookie_result = if let Some(token) = &id_cookie {
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
                latitude.unwrap_or(200.),
                longitude.unwrap_or(200.),
                addr
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
                    expires_at: (chrono::Utc::now() + chrono::Duration::days(6 * 30)).naive_utc(),
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
            latitude,
            longitude,
            country_code,
        };
        let pages_views_repository = PageViewRepo::new(conn);
        let view_insert_result = match pages_views_repository.insert_one(new_view) {
            Err(err) => {
                return Ok(server_error_response(err));
            }
            Ok(value) => value,
        };

        let resp = simple_created_response(view_insert_result);

        if let Some(token) = return_cookie {
            let mut cookie = Cookie::new("identifier", token);
            let mut expiration = OffsetDateTime::now_utc();
            expiration += Duration::days(6 * 30);
            cookie.set_expires(expiration);
            cookie.set_http_only(true);
            cookie.set_path("/");
            cookies.add(cookie);
        }
        Ok(resp)
    })
    .await?)
}
