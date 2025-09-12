use axum::extract::Path;
use axum::{Json, Router, routing::get};
use database::Database;
use database::models::user::UserProfile;
use database::models::user::{User, UserPatchPayload};

use crate::AppState;
use crate::error::AppError;
use crate::extractors::authenticated_user::AuthenticatedUser;

pub struct UserHandler;

impl UserHandler {
    pub fn router() -> Router<AppState> {
        Router::new()
            .route("/me", get(Self::current_user))
            .route("/{id}", get(Self::profile).patch(Self::patch))
    }

    async fn current_user(user: AuthenticatedUser) -> Result<Json<AuthenticatedUser>, AppError> {
        Ok(Json(user))
    }

    async fn profile(
        Path(user_id_or_name): Path<String>,
        db: Database,
    ) -> Result<Json<UserProfile>, AppError> {
        Ok(Json(db.users().profile(user_id_or_name.into()).await?))
    }

    async fn patch(
        Path(user_id): Path<u32>,
        authenticated_user: AuthenticatedUser,
        db: Database,
        Json(payload): Json<UserPatchPayload>,
    ) -> Result<Json<User>, AppError> {
        if user_id != authenticated_user.id {
            return Err(AppError::Forbidden);
        }

        Ok(Json(db.users().patch(user_id, payload).await?))
    }
}
