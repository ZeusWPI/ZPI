use sqlx::SqlitePool;

use crate::{error::DatabaseError, models::services::Service};

pub struct ServiceRepo<'a> {
    db: &'a SqlitePool,
}

impl<'a> ServiceRepo<'a> {
    pub fn new(db: &'a SqlitePool) -> Self {
        Self { db }
    }

    pub async fn all(&self) -> Result<Service, DatabaseError> {
        sqlx::query_as("SELECT id, name FROM service;")
            .fetch_optional(self.db)
            .await?
            .ok_or(DatabaseError::NotFound)

        // TODO fetch_optional?
    }
}
