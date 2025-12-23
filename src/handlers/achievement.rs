use axum::{Json, extract::Path};
use database::Database;

use crate::{
    dto::achievement::{AchievementCreatePayload, AchievementPayload},
    error::AppError,
};

pub struct AchievementHandler;

impl AchievementHandler {
    pub async fn get_for_service(
        db: Database,
        Path(service_id): Path<u32>,
    ) -> Result<Json<Vec<AchievementPayload>>, AppError> {
        Ok(Json(
            AchievementPayload::for_service(&db, service_id).await?,
        ))
    }

    pub async fn post_for_service(
        db: Database,
        Path(service_id): Path<u32>,
        Json(achievement): Json<AchievementCreatePayload>,
    ) -> Result<Json<AchievementPayload>, AppError> {
        if achievement.goals.is_empty() {
            return Err(AppError::PayloadError(
                "Achievement must have at least 1 goal".into(),
            ));
        }

        Ok(Json(achievement.create(service_id, &db).await?))
    }
}
