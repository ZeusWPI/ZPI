use serde::{Deserialize, Serialize};
use sqlx::{Decode, SqlitePool, prelude::FromRow};

use crate::error::AppError;

#[derive(Debug, FromRow, Decode, Serialize, Deserialize, PartialEq)]
pub struct Tag {
    id: u32,
    name: String,
    r#type: String,
    description: Option<String>,
}

impl Tag {
    pub async fn for_user(db: &SqlitePool, id: u32) -> Result<Vec<Tag>, AppError> {
        Ok(sqlx::query_as(
            "SELECT tag.id, tag.name, tag_type.name AS type, description
            FROM user_tag
                INNER JOIN tag
                    ON user_tag.tag_id = tag.id AND user_tag.user_id = ?
                INNER JOIN tag_type;
            ",
        )
        .bind(id)
        .fetch_all(db)
        .await?)
    }
}
