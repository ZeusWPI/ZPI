use rand::RngCore;
use sqlx::SqlitePool;

use crate::{
    error::DatabaseError,
    models::service::{Service, ServiceCreate, ServicePatch},
};

pub struct ServiceRepo<'a> {
    db: &'a SqlitePool,
}

impl<'a> ServiceRepo<'a> {
    pub fn new(db: &'a SqlitePool) -> Self {
        Self { db }
    }

    pub async fn all(&self) -> Result<Vec<Service>, DatabaseError> {
        Ok(sqlx::query_as("SELECT id, name, api_key FROM service;")
            .fetch_all(self.db)
            .await?)
    }

    pub async fn create(&self, service: ServiceCreate) -> Result<Service, DatabaseError> {
        let mut api_key = [0u8; 32];
        rand::rng().fill_bytes(&mut api_key);
        let api_key = base_62::encode(&api_key);

        sqlx::query_as(
            "
       INSERT INTO service (name, api_key) VALUES (?, ?)
       RETURNING id, name, api_key;
       ",
        )
        .bind(service.name)
        .bind(api_key)
        .fetch_optional(self.db)
        .await?
        .ok_or(DatabaseError::NotFound)
    }

    pub async fn patch(
        &self,
        service_id: u32,
        patch_service: ServicePatch,
    ) -> Result<Service, DatabaseError> {
        sqlx::query_as(
            "
        UPDATE service SET name = ? WHERE id = ?
        RETURNING id, name, api_key
        ",
        )
        .bind(patch_service.name)
        .bind(service_id)
        .fetch_optional(self.db)
        .await?
        .ok_or(DatabaseError::NotFound)
    }
}
