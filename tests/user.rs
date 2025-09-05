use database::models::user::{User, UserPatchPayload, UserProfile};
use reqwest::StatusCode;
use sqlx::SqlitePool;
use zpi::handlers::AuthenticatedUser;

use crate::common::{
    into_struct::IntoStruct,
    router::{AuthenticatedRouter, UnauthenticatedRouter},
};

mod common;

#[sqlx::test]
async fn get_users_me(db_pool: SqlitePool) {
    let router = AuthenticatedRouter::new(db_pool).await;
    let response = router.get("/users/me").await;

    assert_eq!(response.status(), StatusCode::OK);

    let user_response: AuthenticatedUser = response.into_struct().await;

    assert_eq!(
        user_response,
        AuthenticatedUser {
            id: 1,
            username: "cheese".into(),
        }
    );
}

#[sqlx::test]
async fn get_users_me_unauthenticated(db_pool: SqlitePool) {
    let router = UnauthenticatedRouter::new(db_pool).await;
    let response = router.get("/users/me").await;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[sqlx::test(fixtures("users"))]
async fn patch_user(db_pool: SqlitePool) {
    let router = AuthenticatedRouter::new(db_pool).await;
    let body = UserPatchPayload {
        about: "Changed about".to_string(),
    };
    let response = router.patch("/users/1", body).await;

    assert_eq!(response.status(), StatusCode::OK);

    let user_response: User = response.into_struct().await;

    assert_eq!(
        user_response,
        User {
            id: 1,
            username: "cheese".into(),
            about: "Changed about".to_string()
        }
    );
}

#[sqlx::test(fixtures("users"))]
async fn get_profile_by_id(db_pool: SqlitePool) {
    let router = AuthenticatedRouter::new(db_pool).await;
    let response = router.get("/users/1").await;

    assert_eq!(response.status(), StatusCode::OK);

    let user_response: UserProfile = response.into_struct().await;

    assert_eq!(
        user_response,
        UserProfile {
            id: 1,
            username: "cheese".into(),
            about: "Just a test user, doing its job... and fantasizing about a life outside the test environment.".to_string(),
            tags: Vec::new(),
        }
    );
}

#[sqlx::test]
async fn get_profile_by_id_unauthenticated(db_pool: SqlitePool) {
    let router = UnauthenticatedRouter::new(db_pool).await;
    let response = router.get("/users/1").await;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[sqlx::test]
async fn get_profile_404(db_pool: SqlitePool) {
    let router = AuthenticatedRouter::new(db_pool.clone()).await;
    let response = router.get("/users/1").await;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let router = AuthenticatedRouter::new(db_pool).await;
    let response = router.get("/users/cheese").await;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[sqlx::test(fixtures("users"))]
async fn get_profile_by_name(db_pool: SqlitePool) {
    let router = AuthenticatedRouter::new(db_pool).await;
    let response = router.get("/users/cheese").await;

    assert_eq!(response.status(), StatusCode::OK);

    let user_response: UserProfile = response.into_struct().await;

    assert_eq!(
        user_response,
        UserProfile {
            id: 1,
            username: "cheese".into(),
            about: "Just a test user, doing its job... and fantasizing about a life outside the test environment.".to_string(),
            tags: Vec::new(),
        }
    );
}
