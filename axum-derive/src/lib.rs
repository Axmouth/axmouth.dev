pub use axum;
use axum::{
    body::BoxBody,
    http::{Response, StatusCode},
    response::IntoResponse,
};
pub use http_body;
pub use validator;

pub use macros::*;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidatedFormRejection {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AxumQueryRejection(#[from] axum::extract::rejection::QueryRejection),
}

impl IntoResponse for ValidatedFormRejection {
    fn into_response(self) -> Response<BoxBody> {
        match self {
            ValidatedFormRejection::ValidationError(err) => {
                (StatusCode::BAD_REQUEST, err.to_string()).into_response()
            }
            ValidatedFormRejection::AxumQueryRejection(err) => err.into_response(),
        }
    }
}
