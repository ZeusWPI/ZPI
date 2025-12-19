use axum::{Json, extract::Path};
use database::Database;

use crate::{
    dto::service::{
        ServiceCreatePayload, ServicePatchPayload, ServicePayloadAdmin, ServicePayloadUser,
    },
    error::AppError,
};

pub struct ServiceHandler;

impl ServiceHandler {
    pub async fn get_admin(db: Database) -> Result<Json<Vec<ServicePayloadAdmin>>, AppError> {
        Ok(Json(ServicePayloadAdmin::all(&db).await?))
    }

    pub async fn get_user(db: Database) -> Result<Json<Vec<ServicePayloadUser>>, AppError> {
        Ok(Json(ServicePayloadUser::all(&db).await?))
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

    pub async fn api_key(
        db: Database,
        Path(service_id): Path<u32>,
    ) -> Result<Json<ServicePayloadAdmin>, AppError> {
        Ok(Json(
            ServicePayloadAdmin::regenerate_api_key(&db, service_id).await?,
        ))
    }
}
