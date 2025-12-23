use reqwest::StatusCode;
use sqlx::SqlitePool;
use zpi::dto::{
    achievement::{AchievementCreatePayload, AchievementPayload},
    goal::GoalCreatePayload,
};

use crate::common::{
    into_struct::IntoStruct,
    router::{AuthenticatedRouter, UnauthenticatedRouter},
    test_objects::TestObjects,
};

mod common;

#[sqlx::test(fixtures("services", "achievements"))]
#[test_log::test]
async fn get_achievements_for_service(db_pool: SqlitePool) {
    let router = AuthenticatedRouter::new(db_pool).await;
    let response = router.get("/admin/services/1/achievements").await;

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
                description: "Get 2 achievements".into(),
                sequence: 1,
            },
            GoalCreatePayload {
                description: "Get 1 achievement".into(),
                sequence: 0,
            },
        ],
    };
    let response = router.post("/admin/services/1/achievements", body).await;

    assert_eq!(response.status(), StatusCode::OK);

    let data: AchievementPayload = response.into_struct().await;

    assert_eq!(data, TestObjects::achievement_1());
}

#[sqlx::test(fixtures("services"))]
#[test_log::test]
async fn post_achievements_wrong_sequence(db_pool: SqlitePool) {
    let router = AuthenticatedRouter::new(db_pool).await;
    let mut body = AchievementCreatePayload {
        name: "Achievements".into(),
        goals: vec![
            GoalCreatePayload {
                description: "Get 2 achievements".into(),
                sequence: 2,
            },
            GoalCreatePayload {
                description: "Get 1 achievement".into(),
                sequence: 0,
            },
        ],
    };

    let response = router
        .clone()
        .post("/admin/services/1/achievements", &body)
        .await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    body.goals[1].sequence = 1;
    let response = router.post("/admin/services/1/achievements", &body).await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[sqlx::test(fixtures("services"))]
#[test_log::test]
async fn unlock_goal_wrong_api_key(db_pool: SqlitePool) {
    let router = UnauthenticatedRouter::new(db_pool)
        .await
        .with_api_key("wrongapikey");

    let response = router.post("/users/1/unlock/1/1", None::<()>).await;
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[sqlx::test(fixtures("services", "achievements", "users"))]
#[test_log::test]
async fn unlock_goal(db_pool: SqlitePool) {
    let router = UnauthenticatedRouter::new(db_pool)
        .await
        .with_api_key("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");

    let response = router.post("/users/1/unlock/1/1", None::<()>).await;
    assert_eq!(response.status(), StatusCode::OK);

    let data: AchievementPayload = response.into_struct().await;

    assert_eq!(data, TestObjects::achievement_1());
}

// TODO wat als goal niet bestaat -> status code 404
// TODO wat als goal al unlocked is -> status code 200
