use axum::extract::{Path, State};
use axum::{Json, Router, routing::get};
use sqlx::SqlitePool;

use crate::models::user::User;
use crate::{error::AppError, handlers::AuthenticatedUser};

pub struct UserHandler;

impl UserHandler {
    pub fn router() -> Router<SqlitePool> {
        Router::new()
            .route("/me", get(Self::current_user))
            .route("/{id}", get(Self::user_with_id))
    }

    async fn current_user(user: AuthenticatedUser) -> Result<Json<AuthenticatedUser>, AppError> {
        Ok(Json(user))
    }

    async fn user_with_id(
        Path(user_id): Path<u32>,
        State(db): State<SqlitePool>,
    ) -> Result<Json<User>, AppError> {
        Ok(Json(User::get_single(&db, user_id).await))
    }
}
