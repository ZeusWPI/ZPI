use database::models::achievement::AchievementPayload;
use reqwest::StatusCode;
use sqlx::SqlitePool;

use crate::common::{
    into_struct::IntoStruct, router::AuthenticatedRouter, test_objects::TestObjects,
};

mod common;

#[sqlx::test(fixtures("services", "achievements"))]
#[test_log::test]
async fn get_achievements_for_service(db_pool: SqlitePool) {
    let router = AuthenticatedRouter::new(db_pool).await;
    let response = router.get("/services/1/achievements").await;

    assert_eq!(response.status(), StatusCode::OK);

    let data: Vec<AchievementPayload> = response.into_struct().await;

    assert_eq!(data, TestObjects::achievement_1_2());
}
