use reqwest::StatusCode;
use sqlx::SqlitePool;
use zpi::dto::{
    achievement::{AchievementCreatePayload, AchievementPayload},
    goal::GoalCreatePayload,
};

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

    assert_eq!(
        data,
        vec![TestObjects::achievement_1(), TestObjects::achievement_2()]
    );
}

#[sqlx::test(fixtures("services"))]
#[test_log::test]
async fn post_achievements_for_service(db_pool: SqlitePool) {
    let router = AuthenticatedRouter::new(db_pool).await;
    let body = AchievementCreatePayload {
        name: "Achievements".into(),
        goals: vec![
            GoalCreatePayload {
                description: "Get 1 achievement".into(),
                sequence: 1,
            },
            GoalCreatePayload {
                description: "Get 2 achievements".into(),
                sequence: 2,
            },
        ],
    };
    let response = router.post("/services/1/achievements", body).await;

    assert_eq!(response.status(), StatusCode::OK);

    let data: AchievementPayload = response.into_struct().await;

    assert_eq!(data, TestObjects::achievement_1());
}
