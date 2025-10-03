use axum::{extract::FromRequestParts, http::request::Parts};

use crate::{error::AppError, extractors::authenticated_user::AuthenticatedUser};

#[derive(Debug)]
pub struct Admin(pub AuthenticatedUser);

impl<S> FromRequestParts<S> for Admin
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let user = AuthenticatedUser::from_request_parts(parts, state).await?;
        if user.admin {
            Ok(Admin(user))
        } else {
            Err(AppError::Forbidden)
        }
    }
}
