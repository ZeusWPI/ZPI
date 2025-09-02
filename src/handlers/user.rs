use axum::extract::{Path, State};
use axum::{Json, Router, routing::get};
use sqlx::SqlitePool;

use crate::models::user::{User, UserPatchPayload};
use crate::{error::AppError, handlers::AuthenticatedUser};

pub struct UserHandler;

impl UserHandler {
    pub fn router() -> Router<SqlitePool> {
        Router::new()
            .route("/me", get(Self::current_user))
            .route("/{id}", get(Self::user_with_id).patch(Self::patch))
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

    async fn patch(
        Path(user_id): Path<u32>,
        authenticated_user: AuthenticatedUser,
        State(db): State<SqlitePool>,
        Json(payload): Json<UserPatchPayload>,
    ) -> Result<Json<User>, AppError> {
        if user_id == authenticated_user.id {
            Ok(Json(payload.update_user(&db, authenticated_user).await))
        } else {
            Err(AppError::Forbidden)
        }
    }
}
