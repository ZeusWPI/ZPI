use axum::{extract::Request, middleware::Next, response::IntoResponse};

use crate::{error::AppError, handlers::AuthenticatedUser};

pub async fn is_logged_in(
    request: Request,
    next: Next,
    _user: AuthenticatedUser,
) -> Result<impl IntoResponse, AppError> {
    Ok(next.run(request).await)
}
