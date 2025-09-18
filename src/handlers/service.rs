use axum::{Json, extract::Path};
use database::{Database, models::service::Service};

use crate::{
    dto::service::{ServiceCreatePayload, ServicePatchPayload, ServicePayloadAdmin},
    error::AppError,
};

pub struct ServiceHandler;

impl ServiceHandler {
    pub async fn get(db: Database) -> Result<Json<Vec<Service>>, AppError> {
        Ok(Json(db.services().all().await?))
    }

    pub async fn post(
        db: Database,
        Json(payload): Json<ServiceCreatePayload>,
    ) -> Result<Json<ServicePayloadAdmin>, AppError> {
        Ok(Json(payload.create(&db).await?))
    }

    pub async fn patch(
        db: Database,
        Path(service_id): Path<u32>,
        Json(payload): Json<ServicePatchPayload>,
    ) -> Result<Json<ServicePayloadAdmin>, AppError> {
        Ok(Json(payload.patch(service_id, &db).await?))
    }
}
