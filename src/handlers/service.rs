use axum::Json;
use database::{Database, models::service::Service};

use crate::error::AppError;

pub struct ServiceHandler;

impl ServiceHandler {
    pub async fn get(db: Database) -> Result<Json<Vec<Service>>, AppError> {
        Ok(Json(db.services().all().await?))
    }
}
