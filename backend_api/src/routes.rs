#[macro_use]
// from https://github.com/seanmonstar/warp/issues/619#issuecomment-662716377
/// Takes a list of handler expressions and `or`s them together
/// in a balanced tree. That is, instead of `a.or(b).or(c).or(d)`,
/// it produces `(a.or(b)).or(c.or(d))`, thus nesting the types
/// less deeply, which provides improvements in compile time.
///
/// It also applies `::warp::Filter::boxed` to each handler expression
/// when in `debug_assertions` mode, improving compile time further.
//
// The basic list splitting algorithm here is based on this gist:
// https://gist.github.com/durka/9fc479de2555225a787f
// It uses a counter from which two items are removed each time,
// stopping when the counter reaches 0. At each step, one item
// is moved from the left to the right, and thus at the end,
// there will be the same number of items in each list.
//
// The flow is as follows:
// - If there is one handler expression, debug_box it and return.
// - If there is more than one handler expression:
//   - First, copy the list into two: the one that will go into the
//     right side of the `or`, and one that will serve as a counter.
//     Recurse with these separated by semicolons, plus an empty `left`
//     list before the first semicolon.
//   - Then, as long as there are at least two items in the counter
//     list, remove them and move the first item on the right side of
//     the first semicolon (`head`) to the left side of the first semicolon.
//   - Finally, when there are one or zero items left in the counter,
//     move one last item to the left, make the call this macro on both the
//     left and right sides, and `or` the two sides together.
//
// For example, balanced_or_tree!(a, b, c, d, e) would take the following steps:
//
// - balanced_or_tree!(a, b, c, d, e)
// - balanced_or_tree!(@internal ; a, b, c, d, e ; a, b, c, d, e) // initialise lists
// - balanced_or_tree!(@internal a ; b, c, d, e ; c, d, e) // move one elem; remove two
// - balanced_or_tree!(@internal a, b ; c, d, e ; e) // now only one elem in counter
// - balanced_or_tree!(a, b, c).or(balanced_or_tree(d, e)) // recurse on each sublist
macro_rules! balanced_or_tree {
    // Base case: just a single expression, return it wrapped in `debug_boxed`
    ($x:expr $(,)?) => { debug_boxed!($x) };
    // Multiple expressions: recurse with three lists: left, right and counter.
    ($($x:expr),+ $(,)?) => {
        balanced_or_tree!(@internal ; $($x),+; $($x),+)
    };
    // Counter 1 or 2; move one more item and recurse on each sublist, and or them together
    (@internal $($left:expr),*; $head:expr, $($tail:expr),+; $a:expr $(,$b:expr)?) => {
        (balanced_or_tree!($($left,)* $head)).or(balanced_or_tree!($($tail),+))
    };
    // Counter > 2; move one item from the right to the left and subtract two from the counter
    (@internal $($left:expr),*; $head:expr, $($tail:expr),+; $a:expr, $b:expr, $($more:expr),+) => {
        balanced_or_tree!(@internal $($left,)* $head; $($tail),+; $($more),+)
    };
}

#[cfg(debug_assertions)]
macro_rules! debug_boxed {
    ($x:expr) => {
        ::warp::Filter::boxed($x)
    };
}

#[cfg(not(debug_assertions))]
macro_rules! debug_boxed {
    ($x:expr) => {
        $x
    };
}

use warp::{self, hyper::StatusCode, Filter, Rejection, Reply};

use crate::app::AppState;
use crate::filters::{auth_admin_filter, auth_filter, auth_opt_filter, validated_json, with_state};
use crate::handlers;
use backend_repo_pg::models::{
    queries::{
        GetAllBlogPostCommentsQuery, GetAllBlogPostsQuery, GetAllCategoriesQuery,
        GetAllHomePageLinksQuery, GetAllProjectsQuery, GetAllTechnologiesQuery,
        GetAllTextBodiesQuery,
    },
    responses::BaseResponse,
};
use std::{convert::Infallible, error::Error};

pub fn routes(
    state: AppState,
) -> impl Filter<Extract = impl warp::Reply, Error = Infallible> + Clone {
    let health_route = warp::path!("health")
        .and(warp::get())
        .and(with_state(state.clone()))
        .and_then(handlers::health::health);

    let get_blog_comment = warp::path!("blog-post-comments" / i32)
        .and(warp::get())
        .and(with_state(state.clone()))
        .and_then(handlers::blog_comments::get);
    let create_blog_comment = warp::path!("blog-post-comments")
        .and(warp::post())
        .and(auth_filter(state.jwt_secret.clone()))
        .and(validated_json())
        .and(with_state(state.clone()))
        .and_then(handlers::blog_comments::create);
    let update_blog_comment = warp::path!("blog-post-comments" / i32)
        .and(warp::put())
        .and(auth_filter(state.jwt_secret.clone()))
        .and(validated_json())
        .and(with_state(state.clone()))
        .and_then(handlers::blog_comments::update);
    let delete_blog_comment = warp::path!("blog-post-comments" / i32)
        .and(warp::delete())
        .and(auth_filter(state.jwt_secret.clone()))
        .and(with_state(state.clone()))
        .and_then(handlers::blog_comments::delete);
    let get_all_blog_comments = warp::path!("blog-post-comments")
        .and(warp::get())
        .and(warp::query::<GetAllBlogPostCommentsQuery>())
        .and(with_state(state.clone()))
        .and_then(handlers::blog_comments::get_all);
    let get_blog_post = warp::path!("blog-posts" / i32)
        .and(warp::get())
        .and(auth_opt_filter(state.jwt_secret.clone()))
        .and(with_state(state.clone()))
        .and_then(handlers::blog_posts::get);
    let create_blog_post = warp::path!("blog-posts")
        .and(warp::post())
        .and(auth_admin_filter(state.jwt_secret.clone()))
        .and(validated_json())
        .and(with_state(state.clone()))
        .and_then(handlers::blog_posts::create);
    let update_blog_post = warp::path!("blog-posts" / i32)
        .and(warp::put())
        .and(auth_admin_filter(state.jwt_secret.clone()))
        .and(validated_json())
        .and(with_state(state.clone()))
        .and_then(handlers::blog_posts::update);
    let delete_blog_post = warp::path!("blog-posts" / i32)
        .and(warp::delete())
        .and(auth_admin_filter(state.jwt_secret.clone()))
        .and(with_state(state.clone()))
        .and_then(handlers::blog_posts::delete);
    let get_all_blog_posts = warp::path!("blog-posts")
        .and(warp::get())
        .and(warp::query::<GetAllBlogPostsQuery>())
        .and(auth_opt_filter(state.jwt_secret.clone()))
        .and(with_state(state.clone()))
        .and_then(handlers::blog_posts::get_all);
    let get_project = warp::path!("projects" / i32)
        .and(warp::get())
        .and(auth_opt_filter(state.jwt_secret.clone()))
        .and(with_state(state.clone()))
        .and_then(handlers::projects::get);
    let create_project = warp::path!("projects")
        .and(warp::post())
        .and(auth_admin_filter(state.jwt_secret.clone()))
        .and(validated_json())
        .and(with_state(state.clone()))
        .and_then(handlers::projects::create);
    let update_project = warp::path!("projects" / i32)
        .and(warp::put())
        .and(auth_admin_filter(state.jwt_secret.clone()))
        .and(validated_json())
        .and(with_state(state.clone()))
        .and_then(handlers::projects::update);
    let delete_project = warp::path!("projects" / i32)
        .and(warp::delete())
        .and(auth_admin_filter(state.jwt_secret.clone()))
        .and(with_state(state.clone()))
        .and_then(handlers::projects::delete);
    let get_all_projects = warp::path!("projects")
        .and(warp::get())
        .and(warp::query::<GetAllProjectsQuery>())
        .and(auth_opt_filter(state.jwt_secret.clone()))
        .and(with_state(state.clone()))
        .and_then(handlers::projects::get_all);
    let get_link = warp::path!("links" / i32)
        .and(warp::get())
        .and(with_state(state.clone()))
        .and_then(handlers::links::get);
    let create_link = warp::path!("links")
        .and(warp::post())
        .and(auth_admin_filter(state.jwt_secret.clone()))
        .and(validated_json())
        .and(with_state(state.clone()))
        .and_then(handlers::links::create);
    let update_link = warp::path!("links" / i32)
        .and(warp::put())
        .and(auth_admin_filter(state.jwt_secret.clone()))
        .and(validated_json())
        .and(with_state(state.clone()))
        .and_then(handlers::links::update);
    let delete_link = warp::path!("links" / i32)
        .and(warp::delete())
        .and(auth_admin_filter(state.jwt_secret.clone()))
        .and(with_state(state.clone()))
        .and_then(handlers::links::delete);
    let get_all_links = warp::path!("links")
        .and(warp::get())
        .and(warp::query::<GetAllHomePageLinksQuery>())
        .and(with_state(state.clone()))
        .and_then(handlers::links::get_all);
    let get_category = warp::path!("categories" / i32)
        .and(warp::get())
        .and(with_state(state.clone()))
        .and_then(handlers::blog_post_categories::get);
    let create_category = warp::path!("categories")
        .and(warp::post())
        .and(auth_admin_filter(state.jwt_secret.clone()))
        .and(validated_json())
        .and(with_state(state.clone()))
        .and_then(handlers::blog_post_categories::create);
    let update_category = warp::path!("categories" / i32)
        .and(warp::put())
        .and(auth_admin_filter(state.jwt_secret.clone()))
        .and(validated_json())
        .and(with_state(state.clone()))
        .and_then(handlers::blog_post_categories::update);
    let delete_category = warp::path!("categories" / i32)
        .and(warp::delete())
        .and(auth_admin_filter(state.jwt_secret.clone()))
        .and(with_state(state.clone()))
        .and_then(handlers::blog_post_categories::delete);
    let get_all_categories = warp::path!("categories")
        .and(warp::get())
        .and(warp::query::<GetAllCategoriesQuery>())
        .and(with_state(state.clone()))
        .and_then(handlers::blog_post_categories::get_all);
    let get_technology = warp::path!("technologies" / i32)
        .and(warp::get())
        .and(with_state(state.clone()))
        .and_then(handlers::project_technologies::get);
    let create_technology = warp::path!("technologies")
        .and(warp::post())
        .and(auth_admin_filter(state.jwt_secret.clone()))
        .and(validated_json())
        .and(with_state(state.clone()))
        .and_then(handlers::project_technologies::create);
    let update_technology = warp::path!("technologies" / i32)
        .and(warp::put())
        .and(auth_admin_filter(state.jwt_secret.clone()))
        .and(validated_json())
        .and(with_state(state.clone()))
        .and_then(handlers::project_technologies::update);
    let delete_technology = warp::path!("technologies" / i32)
        .and(warp::delete())
        .and(auth_admin_filter(state.jwt_secret.clone()))
        .and(with_state(state.clone()))
        .and_then(handlers::project_technologies::delete);
    let get_all_technologies = warp::path!("technologies")
        .and(warp::get())
        .and(warp::query::<GetAllTechnologiesQuery>())
        .and(with_state(state.clone()))
        .and_then(handlers::project_technologies::get_all);
    let get_text_body = warp::path!("text-bodies" / String)
        .and(warp::get())
        .and(with_state(state.clone()))
        .and_then(handlers::text_bodies::get);
    let create_text_body = warp::path!("text-bodies")
        .and(warp::post())
        .and(auth_admin_filter(state.jwt_secret.clone()))
        .and(validated_json())
        .and(with_state(state.clone()))
        .and_then(handlers::text_bodies::create);
    let update_text_body = warp::path!("text-bodies" / String)
        .and(warp::put())
        .and(auth_admin_filter(state.jwt_secret.clone()))
        .and(validated_json())
        .and(with_state(state.clone()))
        .and_then(handlers::text_bodies::update);
    let delete_text_body = warp::path!("text-bodies" / String)
        .and(warp::delete())
        .and(auth_admin_filter(state.jwt_secret.clone()))
        .and(with_state(state.clone()))
        .and_then(handlers::text_bodies::delete);
    let get_all_text_bodies = warp::path!("text-bodies")
        .and(warp::get())
        .and(warp::query::<GetAllTextBodiesQuery>())
        .and(with_state(state.clone()))
        .and_then(handlers::text_bodies::get_all);
    let get_user = warp::path!("users" / i32)
        .and(warp::get())
        .and(with_state(state.clone()))
        .and_then(handlers::users::get);
    let login = warp::path!("auth" / "login")
        .and(warp::post())
        .and(validated_json())
        .and(with_state(state.clone()))
        .and_then(handlers::auth::login);
    let admin_login = warp::path!("auth" / "admin-login")
        .and(warp::post())
        .and(validated_json())
        .and(with_state(state.clone()))
        .and_then(handlers::auth::admin_login);
    let register = warp::path!("auth" / "register")
        .and(warp::post())
        .and(validated_json())
        .and(with_state(state.clone()))
        .and_then(handlers::auth::register);
    let refresh = warp::path!("auth" / "refresh")
        .and(warp::post())
        .and(validated_json())
        .and(warp::cookie("refresh_token"))
        .and(with_state(state.clone()))
        .and_then(handlers::auth::refresh);
    let logout = warp::path!("auth" / "logout")
        .and(warp::delete())
        .and(warp::cookie("refresh_token"))
        .and(with_state(state.clone()))
        .and_then(handlers::auth::logout);
    let contact_email = warp::path!("contact" / "contact-email")
        .and(warp::post())
        .and(validated_json())
        .and(with_state(state.clone()))
        .and_then(handlers::contact::contact_email);

    let image_upload_editorjs = warp::path!("files" / "upload" / "editorjs")
        .and(warp::post())
        .and(warp::body::content_length_limit(1024 * 1024 * 4))
        .and(warp::multipart::form())
        .and(auth_admin_filter(state.jwt_secret.clone()))
        .and(with_state(state.clone()))
        .and_then(handlers::files::editor_js_upload);

    let image_upload = warp::path!("files" / "upload" / "image")
        .and(warp::post())
        .and(warp::body::content_length_limit(1024 * 1024 * 4))
        .and(warp::multipart::form())
        .and(auth_admin_filter(state.jwt_secret.clone()))
        .and(with_state(state.clone()))
        .and_then(handlers::files::editor_js_upload);

    let files_get = warp::path("static").and(warp::fs::dir(state.static_file_dir));

    let handlers = balanced_or_tree!(
        get_blog_post,
        create_blog_post,
        update_blog_post,
        delete_blog_post,
        get_all_blog_posts,
        get_blog_comment,
        create_blog_comment,
        update_blog_comment,
        delete_blog_comment,
        get_all_blog_comments,
        get_project,
        create_project,
        update_project,
        delete_project,
        get_all_projects,
        create_link,
        update_link,
        delete_link,
        get_link,
        get_all_links,
        create_category,
        update_category,
        delete_category,
        get_category,
        get_all_categories,
        create_technology,
        update_technology,
        delete_technology,
        get_technology,
        get_all_technologies,
        create_text_body,
        update_text_body,
        delete_text_body,
        get_text_body,
        get_all_text_bodies,
        get_user,
        login,
        admin_login,
        register,
        refresh,
        logout,
        contact_email,
        image_upload_editorjs,
        image_upload,
        health_route,
    );

    warp::path("api")
        .and(warp::path("v1"))
        .and(handlers)
        .or(files_get)
        .recover(handle_rejection)
}

// This function receives a `Rejection` and tries to return a custom
// value, otherwise simply passes the rejection along.
async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND".to_string();
    } else if let Some(e) = err.find::<crate::filters::RequestValidationFailure>() {
        message = e.get_err();
        code = StatusCode::BAD_REQUEST;
    } else if let Some(e) = err.find::<crate::filters::InvalidJWT>() {
        message = e.get_err();
        code = StatusCode::BAD_REQUEST;
    } else if let Some(e) = err.find::<crate::filters::MissingAuthentication>() {
        message = e.get_err();
        code = StatusCode::UNAUTHORIZED;
    } else if let Some(e) = err.find::<crate::errors::ExpiredAuthentication>() {
        message = e.to_string();
        code = StatusCode::UNAUTHORIZED;
    } else if let Some(e) = err.find::<crate::errors::EmailError>() {
        message = e.to_string();
        code = StatusCode::INTERNAL_SERVER_ERROR;
    } else if let Some(e) = err.find::<crate::errors::FileUploadError>() {
        message = e.to_string();
        code = StatusCode::INTERNAL_SERVER_ERROR;
    } else if let Some(e) = err.find::<crate::errors::CaptchaError>() {
        message = e.to_string();
        code = StatusCode::BAD_REQUEST;
    } else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
        // This error happens if the body could not be deserialized correctly
        // We can use the cause to analyze the error and customize the error message
        message = match e.source() {
            Some(cause) => {
                if cause.to_string().contains("denom") {
                    "FIELD_ERROR: denom".to_string()
                } else {
                    format!("BAD_REQUEST: {}", e.to_string())
                }
            }
            None => e.to_string(),
        };
        code = StatusCode::BAD_REQUEST;
    } else if let Some(e) = err.find::<warp::reject::MethodNotAllowed>() {
        // We can handle a specific error, here METHOD_NOT_ALLOWED,
        // and render it however we want
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = e.to_string();
    } else if let Some(e) = err.find::<warp::reject::InvalidQuery>() {
        code = StatusCode::BAD_REQUEST;
        message = e.to_string();
    } else if let Some(e) = err.find::<warp::reject::MissingCookie>() {
        code = StatusCode::BAD_REQUEST;
        message = e.to_string();
    } else if let Some(e) = err.find::<warp::reject::MissingHeader>() {
        code = StatusCode::BAD_REQUEST;
        message = e.to_string();
    } else {
        // We should have expected this... Just log and say its a 500
        eprintln!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION".to_string();
    }

    let json = warp::reply::json(&BaseResponse::<()> {
        data: None,
        errors: Some(vec![message]),
        messages: None,
        pagination: None,
        success: Some(false),
    });

    Ok(warp::reply::with_status(json, code))
}
