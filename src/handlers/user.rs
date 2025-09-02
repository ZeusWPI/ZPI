use axum::extract::{Path, State};
use axum::{Json, Router, routing::get};
use sqlx::SqlitePool;

use crate::models::user::{User, UserPatchPayload, UserProfilePayload};
use crate::{error::AppError, handlers::AuthenticatedUser};

pub struct UserHandler;

impl UserHandler {
    pub fn router() -> Router<SqlitePool> {
        Router::new()
            .route("/me", get(Self::current_user))
            .route("/{id}", get(Self::profile).patch(Self::patch))
    }

    async fn current_user(user: AuthenticatedUser) -> Result<Json<AuthenticatedUser>, AppError> {
        Ok(Json(user))
    }

    async fn profile(
        Path(user_id_or_name): Path<String>,
        State(db): State<SqlitePool>,
    ) -> Result<Json<UserProfilePayload>, AppError> {
        match user_id_or_name.parse::<u32>() {
            Ok(id) => Ok(Json(UserProfilePayload::get_by_id(&db, id).await?)),
            Err(_) => Ok(Json(
                UserProfilePayload::get_by_username(&db, user_id_or_name).await?,
            )),
        }
    }

    async fn patch(
        Path(user_id): Path<u32>,
        authenticated_user: AuthenticatedUser,
        State(db): State<SqlitePool>,
        Json(payload): Json<UserPatchPayload>,
    ) -> Result<Json<User>, AppError> {
        if user_id != authenticated_user.id {
            return Err(AppError::Forbidden);
        }
        match payload.update_user(&db, authenticated_user).await? {
            Some(user) => Ok(Json(user)),
            None => Err(AppError::NotFound),
        }
    }
}
