use axum::Json;
use database::{Database, models::services::Service};

use crate::error::AppError;

pub struct ServiceHandler;

impl ServiceHandler {
    pub async fn get(db: Database) -> Result<Json<Service>, AppError> {
        Ok(Json(db.services().all().await?))
    }
}
