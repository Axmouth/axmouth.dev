use axum::{
    async_trait,
    body::BoxBody,
    extract::{
        rejection::TypedHeaderRejectionReason, Form, FromRequest, Query, RequestParts, TypedHeader,
    },
    http::Response,
    response::IntoResponse,
    BoxError, Json,
};
use headers::{authorization::Bearer, Authorization};
use jsonwebtoken::{decode, Validation};
use serde::de::DeserializeOwned;

use thiserror::Error;
use validator::Validate;

use crate::{
    app::KEYS,
    auth_tokens::Claims,
    errors::AuthError,
    filters::{validation_errors_to_msg, InvalidJWT},
    util::{bad_request_response, bad_request_response_many},
};

pub struct ClaimsContext {
    pub claims: Claims,
}

pub struct OptClaimsContext {
    pub claims: Option<Claims>,
}

pub struct AdminClaimsContext {
    pub claims: Claims,
}

pub struct OptAdminClaimsContext {
    pub claims: Option<Claims>,
}

fn ensure_admin(claims: Claims) -> Result<Claims, AuthError> {
    if claims.is_admin() {
        Ok(claims)
    } else {
        Err(AuthError::InsufficientPriviledge)
    }
}

async fn get_claims<B>(req: &mut RequestParts<B>) -> Result<Claims, AuthError>
where
    B: Send,
{
    let TypedHeader(Authorization(bearer)) =
        TypedHeader::<Authorization<Bearer>>::from_request(req)
            .await
            .map_err(|e| AuthError::InvalidJWT(InvalidJWT::new(e.to_string())))?;
    // Decode the user data
    decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
        .map_err(|e| AuthError::InvalidJWT(InvalidJWT::new(e.to_string())))
        .map(|data| data.claims)
}

async fn get_claims_opt<B>(req: &mut RequestParts<B>) -> Result<Option<Claims>, AuthError>
where
    B: Send,
{
    let bearer = match TypedHeader::<Authorization<Bearer>>::from_request(req).await {
        Ok(TypedHeader(Authorization(bearer))) => Ok(bearer),
        Err(e) => match e.reason() {
            TypedHeaderRejectionReason::Missing => return Ok(None),
            _ => Err(e),
        },
    }
    .map_err(|e| AuthError::InvalidJWT(InvalidJWT::new(e.to_string())))?;
    // Decode the user data
    decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
        .map_err(|e| AuthError::InvalidJWT(InvalidJWT::new(e.to_string())))
        .map(|data| Some(data.claims))
}

#[async_trait]
impl<B> FromRequest<B> for ClaimsContext
where
    B: Send,
{
    type Rejection = AuthError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        Ok(ClaimsContext {
            claims: get_claims(req).await?,
        })
    }
}

#[async_trait]
impl<B> FromRequest<B> for OptClaimsContext
where
    B: Send,
{
    type Rejection = AuthError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        Ok(OptClaimsContext {
            claims: get_claims_opt(req).await?,
        })
    }
}

#[async_trait]
impl<B> FromRequest<B> for AdminClaimsContext
where
    B: Send,
{
    type Rejection = AuthError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        Ok(AdminClaimsContext {
            claims: ensure_admin(get_claims(req).await?)?,
        })
    }
}

#[async_trait]
impl<B> FromRequest<B> for OptAdminClaimsContext
where
    B: Send,
{
    type Rejection = AuthError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        Ok(OptAdminClaimsContext {
            claims: get_claims_opt(req).await?.map(ensure_admin).transpose()?,
        })
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedForm<T>(pub T);

#[async_trait]
impl<T, B> FromRequest<B> for ValidatedForm<T>
where
    T: DeserializeOwned + Validate,
    B: http_body::Body + Send,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = ServerError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Form(value) = Form::<T>::from_request(req).await?;
        value.validate()?;
        Ok(ValidatedForm(value))
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<T, B> FromRequest<B> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    B: http_body::Body + Send,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = ServerError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req).await?;
        value.validate()?;
        Ok(ValidatedJson(value))
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedQuery<T>(pub T);

#[async_trait]
impl<T, B> FromRequest<B> for ValidatedQuery<T>
where
    T: DeserializeOwned + Validate,
    B: http_body::Body + Send,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = ServerError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Query(value) = Query::<T>::from_request(req).await?;
        value.validate()?;
        Ok(ValidatedQuery(value))
    }
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AxumFormRejection(#[from] axum::extract::rejection::FormRejection),

    #[error(transparent)]
    AxumJsonRejection(#[from] axum::extract::rejection::JsonRejection),

    #[error(transparent)]
    AxumQueryRejection(#[from] axum::extract::rejection::QueryRejection),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response<BoxBody> {
        match self {
            ServerError::ValidationError(err) => bad_request_response_many(
                validation_errors_to_msg(err)
                    .trim()
                    .split('\n')
                    .map(|s| s.to_string())
                    .collect(),
            ),
            ServerError::AxumFormRejection(err) => bad_request_response(err.to_string()),
            ServerError::AxumJsonRejection(err) => bad_request_response(err.to_string()),
            ServerError::AxumQueryRejection(err) => bad_request_response(err.to_string()),
        }
    }
}
