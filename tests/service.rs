use reqwest::StatusCode;
use serde::Deserialize;
use sqlx::SqlitePool;
use zpi::dto::service::{
    ServiceCreatePayload, ServicePatchPayload, ServicePayloadAdmin, ServicePayloadUser,
};

use crate::common::{
    into_struct::IntoStruct, router::AuthenticatedRouter, test_objects::TestObjects,
};

mod common;

#[sqlx::test(fixtures("services"))]
#[test_log::test]
async fn get_all_services_as_admin(db_pool: SqlitePool) {
    let router = AuthenticatedRouter::new(db_pool).await;
    let response = router.get("/admin/services").await;

    assert_eq!(response.status(), StatusCode::OK);

    let data: Vec<ServicePayloadAdmin> = response.into_struct().await;

    assert_eq!(data, TestObjects::admin_services())
}

#[sqlx::test(fixtures("services"))]
#[test_log::test]
async fn get_all_services(db_pool: SqlitePool) {
    let router = AuthenticatedRouter::new(db_pool).await;
    let response = router.get("/services").await;

    assert_eq!(response.status(), StatusCode::OK);

    let data: Vec<ServicePayloadUser> = response.into_struct().await;

    assert_eq!(data, TestObjects::services())
}

#[derive(Deserialize)]
struct ApiKey {
    api_key: Option<String>,
}

#[sqlx::test(fixtures("services"))]
#[test_log::test]
async fn users_dont_see_api_key(db_pool: SqlitePool) {
    let router = AuthenticatedRouter::new(db_pool).await;
    let response = router.get("/services").await;

    assert_eq!(response.status(), StatusCode::OK);

    let data: Vec<ApiKey> = response.into_struct().await;

    assert!(data.into_iter().all(|x| x.api_key.is_none()))
}

#[sqlx::test]
#[test_log::test]
async fn create_service(db_pool: SqlitePool) {
    let router = AuthenticatedRouter::new(db_pool).await;
    let body = ServiceCreatePayload {
        name: "zpi".to_string(),
    };
    let response = router.post("/admin/services", body).await;

    assert_eq!(response.status(), StatusCode::OK);

    let service_response: ServicePayloadAdmin = response.into_struct().await;
    assert_eq!(service_response.id, TestObjects::admin_service_1().id);
    assert_eq!(service_response.name, TestObjects::admin_service_1().name);

    assert_eq!(service_response.api_key.len(), 44);
}

#[sqlx::test(fixtures("services"))]
#[test_log::test]
async fn patch_service(db_pool: SqlitePool) {
    let new_name = "gamification2";
    let router = AuthenticatedRouter::new(db_pool).await;
    let body = ServicePatchPayload {
        name: new_name.to_string(),
    };
    let response = router.patch("/admin/services/1", body).await;

    assert_eq!(response.status(), StatusCode::OK);

    let service_response: ServicePayloadAdmin = response.into_struct().await;

    let mut expected_service = TestObjects::admin_service_1();
    expected_service.name = new_name.to_string();
    assert_eq!(service_response, expected_service);
}

#[sqlx::test(fixtures("services"))]
#[test_log::test]
async fn regenerate_api_key(db_pool: SqlitePool) {
    let router = AuthenticatedRouter::new(db_pool).await;
    let response = router.post("/admin/services/1/apikey", "").await; // empty body

    assert_eq!(response.status(), StatusCode::OK);

    let data: ServicePayloadAdmin = response.into_struct().await;

    assert_ne!(data.api_key, TestObjects::admin_service_1().api_key)
}
