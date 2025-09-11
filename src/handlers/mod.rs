use axum::{
    extract::{FromRequestParts, State},
    http::request::Parts,
};
use database::Database;
use serde::{Deserialize, Serialize};
use tower_sessions::Session;

use crate::{AppState, config::AppConfig, error::AppError, handlers::auth::ZauthUser};

pub mod auth;
pub mod image;
pub mod user;
pub mod version;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct AuthenticatedUser {
    pub id: u32,
    pub username: String,
    pub admin: bool,
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

impl From<ZauthUser> for AuthenticatedUser {
    fn from(user: ZauthUser) -> Self {
        let admin =
            user.roles.contains(&"bestuur".into()) || user.roles.contains(&"zpi_admin".into());
        Self {
            id: user.id,
            username: user.username,
            admin,
        }
    }
}

impl FromRequestParts<AppState> for Database {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Database, Self::Rejection> {
        let State(app_state) = State::<AppState>::from_request_parts(parts, state)
            .await
            .map_err(|_| AppError::Internal("Failed to extract app state".into()))?;

        Ok(app_state.db)
    }
}

impl FromRequestParts<AppState> for AppConfig {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<AppConfig, Self::Rejection> {
        let State(app_state) = State::<AppState>::from_request_parts(parts, state)
            .await
            .map_err(|_| AppError::Internal("Failed to extract app state".into()))?;
        Ok(app_state.config)
    }
}
