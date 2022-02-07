mod test_suite;

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use backend_api::{app, routes};
    use pretty_assertions::assert_eq;
    use tower::ServiceExt;

    macro_rules! test_file {
        ($path:expr) => {};
    }

    macro_rules! api_oneshot_test {
        ($name:ident, $uri:expr, $method:expr, $status:expr, $response:expr) => {
            #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
            async fn $name() {
                dotenv::dotenv().ok();

                let app = routes::router(app::app_state().await);

                let response = app
                    .oneshot(
                        Request::builder()
                            .uri($uri)
                            .method($method)
                            .body(Body::empty())
                            .expect("failed to build request"),
                    )
                    .await
                    .expect("Failed to make request");

                assert_eq!(response.status(), $status);
                let response_body = hyper::body::to_bytes(response.into_body())
                    .await
                    .expect("Failed to read body");
                assert_eq!($response, response_body);
            }
        };
    }

    api_oneshot_test!(
        route_not_found,
        "/does-not-exist",
        "GET",
        StatusCode::NOT_FOUND,
        "{\"success\":false,\"errors\":[\"Route not found\"]}"
    );

    api_oneshot_test!(
        project_not_found,
        "/api/v1/projects/1111",
        "GET",
        StatusCode::NOT_FOUND,
        "{\"success\":false,\"errors\":[\"Project not found\"]}"
    );

    api_oneshot_test!(
        project_not_found_slug,
        "/api/v1/projects/does-not-exist?useSlug=true",
        "GET",
        StatusCode::NOT_FOUND,
        "{\"success\":false,\"errors\":[\"Project not found\"]}"
    );
}
