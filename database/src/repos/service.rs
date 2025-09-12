use sqlx::SqlitePool;

use crate::{
    error::DatabaseError,
    models::service::{Service, ServiceCreatePayload},
};

pub struct ServiceRepo<'a> {
    db: &'a SqlitePool,
}

impl<'a> ServiceRepo<'a> {
    pub fn new(db: &'a SqlitePool) -> Self {
        Self { db }
    }

    pub async fn all(&self) -> Result<Vec<Service>, DatabaseError> {
        Ok(sqlx::query_as("SELECT id, name FROM service;")
            .fetch_all(self.db)
            .await?)
    }

    pub async fn create(&self, service: ServiceCreatePayload) -> Result<Service, DatabaseError> {
        sqlx::query_as(
            "
       INSERT INTO service (name) VALUES (?)
       RETURNING id, name;
       ",
        )
        .bind(service.name)
        .fetch_optional(self.db)
        .await?
        .ok_or(DatabaseError::NotFound)
    }
}
