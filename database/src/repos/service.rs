use sqlx::SqlitePool;

use crate::{error::DatabaseError, models::service::Service};

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
}
