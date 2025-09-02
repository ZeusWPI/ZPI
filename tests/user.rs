use axum::body::to_bytes;
use reqwest::StatusCode;
use sqlx::SqlitePool;
use zpi::{handlers::AuthenticatedUser, models::user::User};

use crate::common::AuthenticatedRouter;

mod common;

#[sqlx::test]
async fn get_users_me(db_pool: SqlitePool) {
    let router = AuthenticatedRouter::new(db_pool).await;
    let response = router.get("/users/me").await;

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body();
    let user_response: AuthenticatedUser =
        serde_json::from_slice(&to_bytes(body, 1000).await.unwrap())
            .expect("response should be valid json");

    assert_eq!(
        user_response,
        AuthenticatedUser {
            id: 1,
            username: "cheese".into(),
        }
    );
}

#[sqlx::test(fixtures("user_1"))]
async fn get_users_id(db_pool: SqlitePool) {
    let router = AuthenticatedRouter::new(db_pool).await;
    let response = router.get("/users/1").await;

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body();
    let user_response: User = serde_json::from_slice(&to_bytes(body, 1000).await.unwrap())
        .expect("response should be valid json");

    assert_eq!(
        user_response,
        User {
            id: 1,
            username: "cheese".into(),
            about: "Just a test user, doing its job... and fantasizing about a life outside the test environment.".to_string(),
        }
    );
}
