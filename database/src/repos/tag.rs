use sqlx::SqlitePool;

use crate::{error::DatabaseError, models::tag::Tag};

pub struct TagRepo<'a> {
    db: &'a SqlitePool,
}

impl<'a> TagRepo<'a> {
    pub fn new(db: &'a SqlitePool) -> Self {
        Self { db }
    }

    pub async fn for_user(&self, user_id: u32) -> Result<Vec<Tag>, DatabaseError> {
        Ok(sqlx::query_as(
            "SELECT tag.id, tag.name, tag_category.name AS category, description
            FROM user_tag
                INNER JOIN tag
                    ON user_tag.tag_id = tag.id AND user_tag.user_id = ?
                INNER JOIN tag_category
                    ON tag.category = tag_category.id;
            ",
        )
        .bind(user_id)
        .fetch_all(self.db)
        .await?)
    }
}
