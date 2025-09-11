use database::models::user::{User, UserPatchPayload, UserProfile};
use reqwest::StatusCode;
use sqlx::SqlitePool;
use zpi::extractors::AuthenticatedUser;

use crate::common::{
    into_struct::IntoStruct,
    router::{AuthenticatedRouter, UnauthenticatedRouter},
    test_objects::TestObjects,
};

mod common;

#[sqlx::test]
#[test_log::test]
async fn get_users_me(db_pool: SqlitePool) {
    let router = AuthenticatedRouter::new(db_pool).await;
    let response = router.get("/users/me").await;
    assert_eq!(response.status(), StatusCode::OK);

    let user_response: AuthenticatedUser = response.into_struct().await;
    assert_eq!(user_response, TestObjects::authenticated_user_1());
}

#[sqlx::test]
#[test_log::test]
async fn get_users_me_unauthenticated(db_pool: SqlitePool) {
    let router = UnauthenticatedRouter::new(db_pool).await;
    let response = router.get("/users/me").await;
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[sqlx::test(fixtures("users"))]
#[test_log::test]
async fn patch_user(db_pool: SqlitePool) {
    let router = AuthenticatedRouter::new(db_pool).await;
    let body = UserPatchPayload {
        about: "Changed about".to_string(),
    };
    let response = router.patch("/users/1", body).await;

    assert_eq!(response.status(), StatusCode::OK);

    let user_response: User = response.into_struct().await;

    let mut expected_user = TestObjects::user_1();
    expected_user.about = "Changed about".to_string();

    assert_eq!(user_response, expected_user);
}

#[sqlx::test(fixtures("users"))]
#[test_log::test]
async fn get_profile_by_id(db_pool: SqlitePool) {
    let router = AuthenticatedRouter::new(db_pool).await;
    let response = router.get("/users/1").await;
    assert_eq!(response.status(), StatusCode::OK);

    let user_response: UserProfile = response.into_struct().await;
    assert_eq!(user_response, TestObjects::user_profile_1());
}

#[sqlx::test]
#[test_log::test]
async fn get_profile_by_id_unauthenticated(db_pool: SqlitePool) {
    let router = UnauthenticatedRouter::new(db_pool).await;
    let response = router.get("/users/1").await;
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[sqlx::test]
#[test_log::test]
async fn get_profile_404(db_pool: SqlitePool) {
    // test getting by id
    let router = AuthenticatedRouter::new(db_pool.clone()).await;
    let response = router.get("/users/1").await;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    // test getting by username
    let router = AuthenticatedRouter::new(db_pool).await;
    let response = router.get("/users/cheese").await;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[sqlx::test(fixtures("users"))]
#[test_log::test]
async fn get_profile_by_name(db_pool: SqlitePool) {
    let router = AuthenticatedRouter::new(db_pool).await;
    let response = router.get("/users/cheese").await;
    assert_eq!(response.status(), StatusCode::OK);

    let user_response: UserProfile = response.into_struct().await;
    assert_eq!(user_response, TestObjects::user_profile_1());
}
