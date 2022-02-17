// derive macros to implement from request
// custom error handling

use crate::{
    app::{
        AppState, CaptchaSecretImpl, DynCaptchaSecret, DynEmailSender, DynJwtDuration,
        DynJwtSecret, DynStaticFileAddress, DynStaticFileDir, EmailSenderImpl, JwtDurationImpl,
        JwtSecretImpl, StaticFileAddressImpl, StaticFileDirImpl,
    },
    handlers::*,
    util::{not_found_response, server_error_response, simple_error_response},
};
use axum::{
    body::{Body, BoxBody, Bytes},
    error_handling::HandleErrorLayer,
    handler::Handler,
    http::{header, Method, Request, Response, StatusCode},
    response::IntoResponse,
    routing::{delete, get, get_service, post},
    AddExtensionLayer, Router,
};
use backend_repo_pg::pg_util::DynRepo;
use std::{sync::Arc, time::Duration};
use tokio::time::error::Elapsed;
use tower::{filter::AsyncFilterLayer, util::AndThenLayer, BoxError, ServiceBuilder};
use tower_cookies::CookieManagerLayer;
use tower_http::{
    cors::{CorsLayer, Origin},
    services::ServeDir,
    trace::TraceLayer,
};

pub fn router(app_state: AppState) -> Router {
    let repo = Arc::new(app_state.repo) as DynRepo;
    let jwt_secret = Arc::new(JwtSecretImpl(app_state.jwt_secret)) as DynJwtSecret;
    let jwt_duration = Arc::new(JwtDurationImpl(app_state.jwt_duration)) as DynJwtDuration;
    let captcha_secret = Arc::new(CaptchaSecretImpl(app_state.captcha_secret)) as DynCaptchaSecret;
    let static_file_dir =
        Arc::new(StaticFileDirImpl(app_state.static_file_dir.clone())) as DynStaticFileDir;
    let static_file_address =
        Arc::new(StaticFileAddressImpl(app_state.static_file_address)) as DynStaticFileAddress;
    let email_sender = Arc::new(EmailSenderImpl(app_state.email_sender)) as DynEmailSender;

    let api_routes = Router::new()
        .route("/health", get(health::health))
        .route("/search", get(search::get_all))
        .route("/admin-logs", get(admin_logs::get_all))
        .route("/admin-logs/:id", get(admin_logs::get))
        .route("/page-views", post(page_views::create))
        .route("/page-views/:url", get(page_views::get))
        .route(
            "/blog-post-comments",
            get(blog_comments::get_all).post(blog_comments::create),
        )
        .route(
            "/blog-post-comments/:id",
            get(blog_comments::get)
                .put(blog_comments::update)
                .delete(blog_comments::delete),
        )
        .route(
            "/blog-posts",
            get(blog_posts::get_all).post(blog_posts::create),
        )
        .route(
            "/blog-posts/:id",
            get(blog_posts::get)
                .put(blog_posts::update)
                .delete(blog_posts::delete),
        )
        .route("/projects", get(projects::get_all).post(projects::create))
        .route(
            "/projects/:id",
            get(projects::get)
                .put(projects::update)
                .delete(projects::delete),
        )
        .route("/links", get(links::get_all).post(links::create))
        .route(
            "/links/:id",
            get(links::get).put(links::update).delete(links::delete),
        )
        .route(
            "/categories",
            get(blog_post_categories::get_all).post(blog_post_categories::create),
        )
        .route(
            "/categories/:id",
            get(blog_post_categories::get)
                .put(blog_post_categories::update)
                .delete(blog_post_categories::delete),
        )
        .route(
            "/technologies",
            get(project_technologies::get_all).post(project_technologies::create),
        )
        .route(
            "/technologies/:id",
            get(project_technologies::get)
                .put(project_technologies::update)
                .delete(project_technologies::delete),
        )
        .route(
            "/text-bodies",
            get(text_bodies::get_all).post(text_bodies::create),
        )
        .route(
            "/text-bodies/:slug",
            get(text_bodies::get)
                .put(text_bodies::update)
                .delete(text_bodies::delete),
        )
        .route("/users/:id", get(users::get))
        .route("/auth/login", post(auth::login))
        .route("/auth/admin-login", post(auth::admin_login))
        .route("/auth/register", post(auth::register))
        .route("/auth/refresh", post(auth::refresh))
        .route("/auth/logout", delete(auth::logout))
        .route("/auth/admin-logout", delete(auth::logout_admin))
        .route("/auth/profile", get(auth::get_profile))
        .route(
            "/auth/request-verification-email",
            post(auth::request_verification_email),
        )
        .route("/auth/verify-email", post(auth::verify_email))
        .route(
            "/auth/request-password-reset",
            post(auth::request_reset_password_email),
        )
        .route("/auth/reset-password", post(auth::reset_password))
        .route("/contact-email", post(contact::contact_email))
        .route("/files/upload/image", post(files::editor_js_upload))
        .route("/files/upload/editorjs", post(files::editor_js_upload))
        // We add middleware
        .layer(AddExtensionLayer::new(repo))
        .layer(AddExtensionLayer::new(jwt_secret))
        .layer(AddExtensionLayer::new(jwt_duration))
        .layer(AddExtensionLayer::new(captcha_secret))
        .layer(AddExtensionLayer::new(static_file_dir))
        .layer(AddExtensionLayer::new(static_file_address))
        .layer(AddExtensionLayer::new(email_sender));

    Router::new()
        .nest("/api/v1", api_routes)
        .nest(
            "/static",
            get_service(ServeDir::new(app_state.static_file_dir)).handle_error(
                |error: std::io::Error| async move {
                    server_error_response(format!("Unhandled internal error: {}", error))
                },
            ),
        )
        .layer(CookieManagerLayer::new())
        .layer(TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .max_age(Duration::from_secs(60) * 10)
                .allow_credentials(true)
                .expose_headers(vec![
                    header::AUTHORIZATION,
                    header::CONTENT_TYPE,
                    header::ORIGIN,
                    header::ACCEPT_LANGUAGE,
                    header::ACCEPT_RANGES,
                    header::ACCESS_CONTROL_ALLOW_ORIGIN,
                    header::ACCESS_CONTROL_ALLOW_HEADERS,
                    header::ACCESS_CONTROL_REQUEST_METHOD,
                    header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
                    header::ACCESS_CONTROL_EXPOSE_HEADERS,
                    header::ACCESS_CONTROL_REQUEST_HEADERS,
                    header::ACCESS_CONTROL_REQUEST_METHOD,
                ])
                .allow_headers(vec![
                    header::AUTHORIZATION,
                    header::CONTENT_TYPE,
                    header::ORIGIN,
                    header::ACCEPT_LANGUAGE,
                    header::ACCEPT_RANGES,
                    header::ACCESS_CONTROL_ALLOW_ORIGIN,
                    header::ACCESS_CONTROL_ALLOW_HEADERS,
                    header::ACCESS_CONTROL_REQUEST_METHOD,
                    header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
                    header::ACCESS_CONTROL_EXPOSE_HEADERS,
                    header::ACCESS_CONTROL_REQUEST_HEADERS,
                    header::ACCESS_CONTROL_REQUEST_METHOD,
                ])
                .allow_methods(vec![
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::PATCH,
                    Method::DELETE,
                    Method::PATCH,
                    Method::CONNECT,
                ])
                .allow_origin(Origin::list(app_state.origin)),
        )
        .fallback(handler_404.into_service())
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|error| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled internal error: {}", error),
                    )
                }))
                .layer(AndThenLayer::new(map_response))
                .layer(AsyncFilterLayer::new(map_request)),
        )
}

async fn handler_404() -> impl IntoResponse {
    not_found_response("Route")
}

async fn handle_error(error: BoxError) -> Response<BoxBody> {
    if error.is::<Elapsed>() {
        simple_error_response("Request timeout", StatusCode::REQUEST_TIMEOUT)
    } else {
        server_error_response(format!("Unhandled internal error: {}", error))
    }
}

async fn map_request(req: Request<Body>) -> Result<Request<Body>, BoxError> {
    let (parts, body) = req.into_parts();
    let bytes = buffer_and_print("request", body).await?;
    let req = Request::from_parts(parts, Body::from(bytes));
    Ok(req)
}

async fn map_response(res: Response<BoxBody>) -> Result<Response<Body>, BoxError> {
    let (parts, body) = res.into_parts();
    let bytes = buffer_and_print("response", body).await?;
    let res = Response::from_parts(parts, Body::from(bytes));
    Ok(res)
}

async fn buffer_and_print<B>(direction: &str, body: B) -> Result<Bytes, BoxError>
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: Into<BoxError>,
{
    let bytes = hyper::body::to_bytes(body).await.map_err(Into::into)?;
    if let Ok(body) = std::str::from_utf8(&bytes) {
        tracing::debug!("{} body = {:?}", direction, body);
    }
    Ok(bytes)
}
