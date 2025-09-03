use axum::extract::{Path, State};
use axum::{Json, Router, routing::get};
use database::models::user::{User, UserPatchPayload};
use database::{Database, models::user::UserProfile};

use crate::{error::AppError, handlers::AuthenticatedUser};

pub struct UserHandler;

impl UserHandler {
    pub fn router() -> Router<Database> {
        Router::new()
            .route("/me", get(Self::current_user))
            .route("/{id}", get(Self::profile).patch(Self::patch))
    }

    async fn current_user(user: AuthenticatedUser) -> Result<Json<AuthenticatedUser>, AppError> {
        Ok(Json(user))
    }

    async fn profile(
        Path(user_id_or_name): Path<String>,
        State(db): State<Database>,
    ) -> Result<Json<UserProfile>, AppError> {
        match user_id_or_name.parse::<u32>() {
            Ok(id) => Ok(Json(db.users().profile_by_id(id).await?)),
            Err(_) => Ok(Json(db.users().profile_by_username(user_id_or_name).await?)),
        }
    }

    async fn patch(
        Path(user_id): Path<u32>,
        authenticated_user: AuthenticatedUser,
        State(db): State<Database>,
        Json(payload): Json<UserPatchPayload>,
    ) -> Result<Json<User>, AppError> {
        if user_id != authenticated_user.id {
            return Err(AppError::Forbidden);
        }

        Ok(Json(db.users().patch(user_id, payload).await?))
    }
}
