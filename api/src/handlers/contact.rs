use crate::app::{DynCaptchaSecret, DynEmailSender};
use crate::errors::AppError;
use crate::extractors::ValidatedJson;
use crate::{errors::CaptchaError, util::simple_created_response};
use axum::extract::Extension;
use axum::response::IntoResponse;
use backend_repo_pg::models::requests::SendContactEmailRequest;
use hyper::{Body, Client, Method, Request};
use tokio::task::block_in_place;

pub async fn contact_email(
    ValidatedJson(request): ValidatedJson<SendContactEmailRequest>,
    Extension(captcha_secret): Extension<DynCaptchaSecret>,
    Extension(email_sender): Extension<DynEmailSender>,
) -> Result<impl IntoResponse, AppError> {
    let captcha_req_body = format!(
        "response={}&secret={}",
        request.captcha_token,
        captcha_secret.captcha_secret()
    );
    let https = hyper_tls::HttpsConnector::new();
    let captcha_request = Request::builder()
        .method(Method::POST)
        .uri("http://hcaptcha.com/siteverify")
        .header("content-type", "application/x-www-form-urlencoded")
        .body(Body::from(captcha_req_body))
        .map_err(CaptchaError::from)?;

    let client = Client::builder().build(https);
    let resp = client
        .request(captcha_request)
        .await
        .map_err(CaptchaError::from)?;
    if resp.status().is_client_error() || resp.status().is_server_error() {
        return Err(AppError::CaptchaError(CaptchaError::new(
            "Error verifying captcha",
        )));
    }

    block_in_place(|| {
        email_sender.email_sender().send_contact_email(
            request.from_email,
            request.subject,
            request.body,
        )
    })?;
    Ok(simple_created_response(1))
}
