use axum::{extract::FromRequestParts, http::request::Parts};
use tower_sessions::Session;

use crate::{error::AppError, handlers::auth::ZauthUser};

pub mod auth;
pub mod image;

#[allow(dead_code)]
pub struct AuthenticatedUser {
    id: u32,
    name: String,
}

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let session = Session::from_request_parts(parts, _state)
            .await
            .map_err(|(_, msg)| AppError::Internal(msg.into()))?;

        let user: Option<ZauthUser> = session.get("user").await.map_err(AppError::Session)?;
        Ok(user.ok_or(AppError::NotLoggedIn)?.into())
    }
}

impl From<ZauthUser> for AuthenticatedUser {
    fn from(user: ZauthUser) -> Self {
        Self {
            id: user.id,
            name: user.username,
        }
    }
}
