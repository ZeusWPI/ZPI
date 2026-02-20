use reqwest::StatusCode;
use sqlx::SqlitePool;
use zpi::dto::user::UserProfile;

use crate::common::{into_struct::IntoStruct, router::TestRouter, test_objects::TestObjects};

mod common;

#[sqlx::test(fixtures("users", "services", "achievements", "unlocks", "tags"))]
#[test_log::test]
async fn get_profile_by_id(db_pool: SqlitePool) {
    let router = TestRouter::as_user(db_pool).await;
    let response = router.get("/users/1").await;
    assert_eq!(response.status(), StatusCode::OK);

    let user_response: UserProfile = response.into_struct().await;
    assert_eq!(user_response, TestObjects::user_profile_1());
}

#[sqlx::test(fixtures("users", "services", "achievements", "unlocks", "tags"))]
#[test_log::test]
async fn get_profile_by_id_with_tags(db_pool: SqlitePool) {
    let router = TestRouter::as_user(db_pool).await;
    let response = router.get("/users/2").await;
    assert_eq!(response.status(), StatusCode::OK);

    let user_response: UserProfile = response.into_struct().await;
    assert_eq!(user_response, TestObjects::user_profile_2());
}

#[sqlx::test]
#[test_log::test]
async fn get_profile_by_id_unauthenticated(db_pool: SqlitePool) {
    let router = TestRouter::new(db_pool);
    let response = router.get("/users/1").await;
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[sqlx::test]
#[test_log::test]
async fn get_profile_404(db_pool: SqlitePool) {
    let router = TestRouter::as_user(db_pool).await;

    // test getting by id
    let response = router.get("/users/1").await;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    // test getting by username
    let response = router.get("/users/cheese").await;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[sqlx::test(fixtures("users", "services", "achievements", "unlocks"))]
#[test_log::test]
async fn get_profile_by_name(db_pool: SqlitePool) {
    let router = TestRouter::as_user(db_pool).await;
    let response = router.get("/users/cheese").await;
    assert_eq!(response.status(), StatusCode::OK);

    let user_response: UserProfile = response.into_struct().await;
    assert_eq!(user_response, TestObjects::user_profile_1());
}
