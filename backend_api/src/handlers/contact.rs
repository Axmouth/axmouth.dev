use crate::app::AppState;
use crate::{
    errors::CaptchaError,
    util::{simple_created_response, simple_error_response},
};
use backend_repo_pg::models::requests::SendContactEmailRequest;
use tokio::task::block_in_place;
use warp::hyper::{Body, Client, Method, Request};

pub async fn contact_email(
    request: SendContactEmailRequest,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let captcha_req_body = format!(
        "response={}&secret={}",
        request.captcha_token, state.captcha_secret
    );
    let https = hyper_tls::HttpsConnector::new();
    let captcha_request = Request::builder()
        .method(Method::POST)
        .uri("http://hcaptcha.com/siteverify")
        .header("content-type", "application/x-www-form-urlencoded")
        .body(Body::from(captcha_req_body))
        .map_err(|err| CaptchaError::from(err))?;

    let client = Client::builder().build(https);
    let resp = client
        .request(captcha_request)
        .await
        .map_err(|err| CaptchaError::from(err))?;
    if resp.status().is_client_error() || resp.status().is_server_error() {
        return Err(warp::reject::custom(CaptchaError::new(
            "Error verifying captcha",
        )));
    }

    block_in_place(|| {
        state
            .email_sender
            .send_contact_email(request.from_email, request.subject, request.body)
    })?;
    Ok(simple_created_response(1))
}
