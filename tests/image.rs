use reqwest::StatusCode;
use sqlx::SqlitePool;

use crate::common::router::TestRouter;

mod common;

#[sqlx::test]
async fn get_image_default(db_pool: SqlitePool) {
    let router = TestRouter::new(db_pool);
    let response = router.get("/image/1").await;

    assert_eq!(response.status(), StatusCode::OK);
}

#[sqlx::test]
async fn get_image_placeholder(db_pool: SqlitePool) {
    let router = TestRouter::new(db_pool);
    let response = router.get("/image/1?placeholder=true").await;

    assert_eq!(response.status(), StatusCode::OK);
}

#[sqlx::test]
async fn get_image_no_placeholder_404(db_pool: SqlitePool) {
    let router = TestRouter::new(db_pool);
    let response = router.get("/image/1?placeholder=false").await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[sqlx::test]
async fn get_image_no_placeholder(db_pool: SqlitePool) {
    let router = TestRouter::new(db_pool);
    let response = router.get("/image/2?placeholder=false").await;

    assert_eq!(response.status(), StatusCode::OK);
}
