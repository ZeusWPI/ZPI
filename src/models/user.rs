use sqlx::{FromRow, SqlitePool};

use crate::handlers::auth::ZauthUser;

#[derive(Debug, FromRow)]
pub struct User {
    pub id: u32,
    pub username: String,
}

impl User {
    pub fn new(id: u32, username: String) -> Self {
        User { id, username }
    }

    pub async fn get_single(db: &SqlitePool, id: u32) -> Self {
        sqlx::query_as("SELECT id, username FROM user WHERE id == ? LIMIT 1;")
            .bind(id)
            .fetch_one(db)
            .await
            .unwrap()
    }

    pub async fn create(&self, db: &SqlitePool) {
        sqlx::query("INSERT INTO user (id, username) VALUES (?, ?);")
            .bind(self.id)
            .bind(&self.username)
            .execute(db)
            .await
            .expect("insert failed");
    }
}

impl From<ZauthUser> for User {
    fn from(value: ZauthUser) -> Self {
        Self::new(value.id, value.username)
    }
}
