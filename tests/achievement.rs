use reqwest::StatusCode;
use sqlx::SqlitePool;
use zpi::dto::{
    achievement::{AchievementCreatePayload, AchievementPayload},
    goal::GoalCreatePayload,
};

use crate::common::{into_struct::IntoStruct, router::TestRouter, test_objects::TestObjects};

mod common;

#[sqlx::test(fixtures("services", "achievements"))]
#[test_log::test]
async fn get_achievements_for_service(db: SqlitePool) {
    let none = TestRouter::new(db.clone());
    let response = none.get("/admin/services/1/achievements").await;
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    let user = TestRouter::as_user(db.clone()).await;
    let response = user.get("/admin/services/1/achievements").await;
    assert_eq!(response.status(), StatusCode::FORBIDDEN);

    let admin = TestRouter::as_admin(db).await;
    let response = admin.get("/admin/services/1/achievements").await;
    assert_eq!(response.status(), StatusCode::OK);

    let data: Vec<AchievementPayload> = response.into_struct().await;
    assert_eq!(
        data,
        vec![TestObjects::achievement_1(), TestObjects::achievement_2()]
    );
}

#[sqlx::test(fixtures("services"))]
#[test_log::test]
async fn post_achievements_for_service(db: SqlitePool) {
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

    let none = TestRouter::new(db.clone());
    let response = none.post("/admin/services/1/achievements", &body).await;
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    let user = TestRouter::as_user(db.clone()).await;
    let response = user.post("/admin/services/1/achievements", &body).await;
    assert_eq!(response.status(), StatusCode::FORBIDDEN);

    let admin = TestRouter::as_admin(db).await;
    let response = admin.post("/admin/services/1/achievements", &body).await;
    assert_eq!(response.status(), StatusCode::OK);

    let data: AchievementPayload = response.into_struct().await;
    assert_eq!(data, TestObjects::achievement_1());
}

#[sqlx::test(fixtures("services"))]
#[test_log::test]
async fn post_achievements_wrong_sequence(db: SqlitePool) {
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

    let router = TestRouter::as_admin(db.clone()).await;
    let response = router.post("/admin/services/1/achievements", &body).await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    body.goals[1].sequence = 1;
    let response = router.post("/admin/services/1/achievements", &body).await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[sqlx::test(fixtures("services", "achievements", "users"))]
#[test_log::test]
async fn unlock_goal(db: SqlitePool) {
    let none = TestRouter::new(db.clone());
    let response = none.post("/users/1/unlock/1/1", None::<()>).await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST); // TODO

    let router = TestRouter::with_api_key(db, "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
    let response = router.post("/users/1/unlock/1/1", None::<()>).await;
    assert_eq!(response.status(), StatusCode::OK);

    let data: AchievementPayload = response.into_struct().await;
    assert_eq!(data, TestObjects::achievement_1());
}

#[sqlx::test(fixtures("services"))]
#[test_log::test]
async fn unlock_goal_wrong_api_key(db_pool: SqlitePool) {
    let router = TestRouter::with_api_key(db_pool, "wrongapikey");

    let response = router.post("/users/1/unlock/1/1", None::<()>).await;
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

// TODO wat als goal niet bestaat -> status code 404
// TODO wat als goal al unlocked is -> status code 200
