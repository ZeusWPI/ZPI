use serde::{Deserialize, Serialize};
use sqlx::{Decode, SqlitePool, prelude::FromRow};

use crate::error::AppError;

#[derive(Debug, FromRow, Decode, Serialize, Deserialize, PartialEq)]
pub struct Tag {
    id: u32,
    name: String,
    description: Option<String>,
}

impl Tag {
    pub async fn for_user(db: &SqlitePool, id: u32) -> Result<Vec<Tag>, AppError> {
        Ok(sqlx::query_as(
            "SELECT id, name, description FROM user_tag INNER JOIN tag WHERE user_id = ?",
        )
        .bind(id)
        .fetch_all(db)
        .await?)
    }
}
