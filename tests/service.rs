use reqwest::StatusCode;
use sqlx::SqlitePool;
use zpi::dto::service::{ServiceCreatePayload, ServicePatchPayload, ServicePayloadAdmin};

use crate::common::{
    into_struct::IntoStruct, router::AuthenticatedRouter, test_objects::TestObjects,
};

mod common;

#[sqlx::test(fixtures("services"))]
#[test_log::test]
async fn get_all_services(db_pool: SqlitePool) {
    let router = AuthenticatedRouter::new(db_pool).await;
    let response = router.get("/admin/services").await;

    assert_eq!(response.status(), StatusCode::OK);

    let data: Vec<ServicePayloadAdmin> = response.into_struct().await;

    assert_eq!(data, TestObjects::services())
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
    assert_eq!(service_response.id, TestObjects::service_1().id);
    assert_eq!(service_response.name, TestObjects::service_1().name);

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

    let mut expected_service = TestObjects::service_1();
    expected_service.name = new_name.to_string();
    assert_eq!(service_response, expected_service);
}
