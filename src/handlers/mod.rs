use axum::{extract::FromRequestParts, http::request::Parts};
use database::models::user::User;
use serde::{Deserialize, Serialize};
use tower_sessions::Session;

use crate::error::AppError;

pub mod auth;
pub mod image;
pub mod user;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct AuthenticatedUser {
    pub id: u32,
    pub username: String,
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

        let user: Option<AuthenticatedUser> =
            session.get("user").await.map_err(AppError::Session)?;
        user.ok_or(AppError::NotLoggedIn)
    }
}

impl From<User> for AuthenticatedUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
        }
    }
}
