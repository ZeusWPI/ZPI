use axum::{Json, extract::Path};
use database::{
    Database,
    models::service::{Service, ServiceCreatePayload, ServicePatchPayload},
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

    pub async fn patch(
        db: Database,
        Path(service_id): Path<u32>,
        Json(payload): Json<ServicePatchPayload>,
    ) -> Result<Json<Service>, AppError> {
        Ok(Json(db.services().patch(service_id, payload).await?))
    }
}
