use axum::{routing::get, Json, Router};
use sqlx::SqlitePool;

use crate::{error::AppError, handlers::AuthenticatedUser};

pub struct UserHandler;

impl UserHandler {
    pub fn router() -> Router<SqlitePool> {
        Router::new().route("/me", get(Self::current_user))
    }

    async fn current_user(
        user: AuthenticatedUser,
    ) -> Result<Json<AuthenticatedUser>, AppError> {
        Ok(Json(user))
    }
}
 