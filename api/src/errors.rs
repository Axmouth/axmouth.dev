use axum::{
    body::BoxBody,
    extract::multipart::MultipartError,
    http::{self, Response, StatusCode},
    response::IntoResponse,
    Json,
};
use backend_repo_pg::{errors::PgRepoError, models::responses::BaseResponse};
use validator::ValidationErrors;

use crate::{filters::InvalidJWT, util::not_found_response};

pub enum AppError {
    PgRepoError(PgRepoError),
    FileUploadError(FileUploadError),
    EmailError(EmailError),
    CaptchaError(CaptchaError),
    ExpiredAuthentication(ExpiredAuthentication),
    GeolocError(GeolocError),
    ValidationErrors(ValidationErrors),
    NotFound(&'static str),
}

impl From<PgRepoError> for AppError {
    fn from(err: PgRepoError) -> Self {
        AppError::PgRepoError(err)
    }
}

impl From<FileUploadError> for AppError {
    fn from(err: FileUploadError) -> Self {
        AppError::FileUploadError(err)
    }
}

impl From<EmailError> for AppError {
    fn from(err: EmailError) -> Self {
        AppError::EmailError(err)
    }
}

impl From<CaptchaError> for AppError {
    fn from(err: CaptchaError) -> Self {
        AppError::CaptchaError(err)
    }
}

impl From<ExpiredAuthentication> for AppError {
    fn from(err: ExpiredAuthentication) -> Self {
        AppError::ExpiredAuthentication(err)
    }
}

impl From<GeolocError> for AppError {
    fn from(err: GeolocError) -> Self {
        AppError::GeolocError(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response<BoxBody> {
        let code;
        let message;

        match self {
            AppError::PgRepoError(err) => {
                match err.error_type {
                    backend_repo_pg::errors::PgRepoErrorType::Conflict => {
                        message = "Conflict".to_string();
                        code = StatusCode::CONFLICT;
                    }
                    backend_repo_pg::errors::PgRepoErrorType::NotFound => {
                        message = "Not found".to_string();
                        code = StatusCode::NOT_FOUND;
                    }
                    backend_repo_pg::errors::PgRepoErrorType::Unknown => {
                        message = "Something went wrong".to_string();
                        code = StatusCode::INTERNAL_SERVER_ERROR;
                    }
                };
            }
            AppError::FileUploadError(err) => {
                message = format!("File Upload: {}", err.to_string());
                code = StatusCode::INTERNAL_SERVER_ERROR;
            }
            AppError::EmailError(err) => {
                message = format!("Email: {}", err.to_string());
                code = StatusCode::INTERNAL_SERVER_ERROR;
            }
            AppError::CaptchaError(err) => {
                message = format!("Captcha: {}", err.to_string());
                code = StatusCode::BAD_REQUEST;
            }
            AppError::ExpiredAuthentication(err) => {
                message = format!("Authentication: {}", err.to_string());
                code = StatusCode::UNAUTHORIZED;
            }
            AppError::GeolocError(err) => {
                message = format!("Geoloc: {}", err.to_string());
                code = StatusCode::INTERNAL_SERVER_ERROR;
            }
            AppError::ValidationErrors(err) => {
                message = format!("Captcha: {}", err);
                code = StatusCode::BAD_REQUEST;
            }
            AppError::NotFound(entity) => {
                return not_found_response(entity);
            }
        }

        let response_body = &BaseResponse::<()> {
            data: None,
            errors: Some(
                message
                    .trim()
                    .split('\n')
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
            ),
            messages: None,
            pagination: None,
            success: Some(false),
        };

        (code, Json(response_body)).into_response()
    }
}

pub enum AuthError {
    ExpiredAuthentication(ExpiredAuthentication),
    InvalidJWT(InvalidJWT),
    InsufficientPriviledge,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response<BoxBody> {
        let code;
        let message;

        match self {
            AuthError::ExpiredAuthentication(err) => {
                message = format!("Authentication: {}", err.err);
                code = StatusCode::UNAUTHORIZED;
            }
            AuthError::InvalidJWT(err) => {
                message = format!("Authentication: {}", err.get_err());
                code = StatusCode::UNAUTHORIZED;
            }
            AuthError::InsufficientPriviledge => {
                message = "Authentication: You are not authorized to do this".to_string();
                code = StatusCode::UNAUTHORIZED;
            }
        }

        let response_body = &BaseResponse::<()> {
            data: None,
            errors: Some(
                message
                    .trim()
                    .split('\n')
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
            ),
            messages: None,
            pagination: None,
            success: Some(false),
        };

        (code, Json(response_body)).into_response()
    }
}

// FileUploadError
#[derive(Debug, Clone)]
pub struct FileUploadError {
    err_message: String,
}

impl FileUploadError {
    pub fn new(err_message: String) -> Self {
        Self { err_message }
    }
}

impl ToString for FileUploadError {
    fn to_string(&self) -> String {
        self.err_message.clone()
    }
}

impl From<std::io::Error> for FileUploadError {
    fn from(error: std::io::Error) -> FileUploadError {
        FileUploadError {
            err_message: error.to_string(),
        }
    }
}

impl From<MultipartError> for FileUploadError {
    fn from(error: MultipartError) -> FileUploadError {
        FileUploadError {
            err_message: error.to_string(),
        }
    }
}

// EmailError
#[derive(Debug)]
pub struct EmailError {
    err: String,
}

impl ToString for EmailError {
    fn to_string(&self) -> String {
        self.err.clone()
    }
}

impl From<lettre_email::error::Error> for EmailError {
    fn from(error: lettre_email::error::Error) -> EmailError {
        EmailError {
            err: error.to_string(),
        }
    }
}

impl From<lettre::smtp::error::Error> for EmailError {
    fn from(error: lettre::smtp::error::Error) -> EmailError {
        EmailError {
            err: error.to_string(),
        }
    }
}

impl From<native_tls::Error> for EmailError {
    fn from(error: native_tls::Error) -> EmailError {
        EmailError {
            err: error.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct CaptchaError {
    err_message: String,
}

impl CaptchaError {
    pub fn get_err(&self) -> String {
        self.err_message.clone()
    }

    pub fn new<D: std::fmt::Display>(err: D) -> Self {
        CaptchaError {
            err_message: err.to_string(),
        }
    }
}

impl ToString for CaptchaError {
    fn to_string(&self) -> String {
        self.err_message.clone()
    }
}

impl From<http::Error> for CaptchaError {
    fn from(error: http::Error) -> CaptchaError {
        CaptchaError {
            err_message: error.to_string(),
        }
    }
}

impl From<hyper::Error> for CaptchaError {
    fn from(error: hyper::Error) -> CaptchaError {
        CaptchaError {
            err_message: error.to_string(),
        }
    }
}

// EmailError
#[derive(Debug)]
pub struct ExpiredAuthentication {
    err: String,
}

impl ExpiredAuthentication {
    pub fn new(err: String) -> Self {
        Self { err }
    }
}

impl ToString for ExpiredAuthentication {
    fn to_string(&self) -> String {
        self.err.clone()
    }
}

#[derive(Debug)]
pub struct GeolocError {
    err_message: String,
}

impl GeolocError {
    pub fn get_err(&self) -> String {
        self.err_message.clone()
    }

    pub fn new<D: std::fmt::Display>(err: D) -> Self {
        GeolocError {
            err_message: err.to_string(),
        }
    }
}

impl ToString for GeolocError {
    fn to_string(&self) -> String {
        self.err_message.clone()
    }
}

impl From<http::Error> for GeolocError {
    fn from(error: http::Error) -> GeolocError {
        GeolocError {
            err_message: error.to_string(),
        }
    }
}

impl From<hyper::Error> for GeolocError {
    fn from(error: hyper::Error) -> GeolocError {
        GeolocError {
            err_message: error.to_string(),
        }
    }
}

impl From<serde_json::Error> for GeolocError {
    fn from(error: serde_json::Error) -> GeolocError {
        GeolocError {
            err_message: error.to_string(),
        }
    }
}
