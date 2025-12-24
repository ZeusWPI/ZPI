use axum::{Json, extract::Path};
use database::Database;

use crate::{
    dto::{
        achievement::AchievementPayload,
        service::{
            ServiceCreatePayload, ServicePatchPayload, ServicePayloadAdmin, ServicePayloadUser,
        },
    },
    error::AppError,
    extractors::api_key::ApiKey,
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

    pub async fn unlock_goal(
        db: Database,
        Path((user_id, service_id, goal_id)): Path<(u32, u32, u32)>,
        ApiKey(api_key): ApiKey,
    ) -> Result<Json<AchievementPayload>, AppError> {
        let expected_api_key = db.services().by_id(service_id).await?.api_key;
        if api_key != expected_api_key {
            return Err(AppError::BadApiKey);
        }

        Ok(Json(
            AchievementPayload::unlock_goal(&db, user_id, goal_id).await?,
        ))
    }
}
