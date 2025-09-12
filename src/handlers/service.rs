use axum::Json;
use database::{
    Database,
    models::service::{Service, ServiceCreatePayload},
};

use crate::error::AppError;

pub struct ServiceHandler;

impl ServiceHandler {
    pub async fn get(db: Database) -> Result<Json<Vec<Service>>, AppError> {
        Ok(Json(db.services().all().await?))
    }

    pub async fn post(
        db: Database,
        Json(payload): Json<ServiceCreatePayload>,
    ) -> Result<Json<Service>, AppError> {
        Ok(Json(db.services().create(payload).await?))
    }
}
