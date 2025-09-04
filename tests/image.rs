use reqwest::StatusCode;
use sqlx::SqlitePool;

use crate::common::UnauthenticatedRouter;

mod common;

#[sqlx::test]
async fn get_image(db_pool: SqlitePool) {
    let router = UnauthenticatedRouter::new(db_pool).await;
    let response = router.get("/image/1").await;

    assert_eq!(response.status(), StatusCode::OK);
}
