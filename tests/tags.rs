use reqwest::StatusCode;
use sqlx::SqlitePool;
use zpi::dto::user::UserProfile;

use crate::common::{into_struct::IntoStruct, router::TestRouter, test_objects::TestObjects};

mod common;

#[sqlx::test(fixtures("users", "tags"))]
#[test_log::test]
async fn get_user_with_tags(db_pool: SqlitePool) {
    let router = TestRouter::as_user(db_pool).await;
    let response = router.get("/users/2").await;

    assert_eq!(response.status(), StatusCode::OK);

    let data: UserProfile = response.into_struct().await;

    assert_eq!(data, TestObjects::user_profile_2())
}
