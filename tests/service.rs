use database::models::service::{Service, ServiceCreatePayload};
use reqwest::StatusCode;
use sqlx::SqlitePool;

use crate::common::{
    into_struct::IntoStruct, router::AuthenticatedRouter, test_objects::TestObjects,
};

mod common;

#[sqlx::test(fixtures("services"))]
#[test_log::test]
async fn get_all_services(db_pool: SqlitePool) {
    let router = AuthenticatedRouter::new(db_pool).await;
    let response = router.get("/services").await;

    assert_eq!(response.status(), StatusCode::OK);

    let data: Vec<Service> = response.into_struct().await;

    assert_eq!(data, TestObjects::services())
}

#[sqlx::test]
#[test_log::test]
async fn create_service(db_pool: SqlitePool) {
    let router = AuthenticatedRouter::new(db_pool).await;
    let body = ServiceCreatePayload {
        name: "zpi".to_string(),
    };
    let response = router.post("/services", body).await;

    assert_eq!(response.status(), StatusCode::OK);

    let service_response: Service = response.into_struct().await;

    assert_eq!(service_response, TestObjects::service_1());
}
