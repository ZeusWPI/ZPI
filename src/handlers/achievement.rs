use axum::{Json, extract::Path};
use database::{Database, models::achievement::AchievementPayload};

use crate::error::AppError;

pub struct AchievementHandler;

impl AchievementHandler {
    pub async fn get_for_service(
        db: Database,
        Path(id): Path<u32>,
    ) -> Result<Json<Vec<AchievementPayload>>, AppError> {
        Ok(Json(db.achievements().for_service(id).await?))
    }
}
