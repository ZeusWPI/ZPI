use axum::{extract::FromRequestParts, http::request::Parts};
use axum_extra::TypedHeader;
use headers::{Authorization, authorization::Bearer};

use crate::error::AppError;

#[derive(Debug)]
pub struct ApiKey(pub String);

impl<S> FromRequestParts<S> for ApiKey
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let header = TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state).await;

        match header {
            Ok(TypedHeader(Authorization(bearer))) => Ok(ApiKey(bearer.token().to_string())),
            _ => Err(AppError::BadApiKey),
        }
    }
}
