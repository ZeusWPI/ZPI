use axum::Json;

use crate::{error::AppError, handlers::AuthenticatedUser};

pub struct UserHandler;

impl UserHandler {
    pub async fn current_user(
        user: AuthenticatedUser,
    ) -> Result<Json<AuthenticatedUser>, AppError> {
        Ok(Json(user))
    }
}
