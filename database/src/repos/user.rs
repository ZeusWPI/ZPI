use sqlx::SqlitePool;

use crate::{
    error::DatabaseError,
    models::user::{User, UserCreate, UserPatch},
};

pub struct UserRepo<'a> {
    db: &'a SqlitePool,
}

impl<'a> UserRepo<'a> {
    pub fn new(db: &'a SqlitePool) -> Self {
        Self { db }
    }

    pub async fn by_id(&self, id: u32) -> Result<User, DatabaseError> {
        sqlx::query_as("SELECT id, username, about FROM user WHERE id == ? LIMIT 1;")
            .bind(id)
            .fetch_optional(self.db)
            .await?
            .ok_or(DatabaseError::NotFound)
    }

    pub async fn by_username(&self, username: String) -> Result<User, DatabaseError> {
        sqlx::query_as("SELECT id, username, about FROM user WHERE username == ? LIMIT 1;")
            .bind(username)
            .fetch_optional(self.db)
            .await?
            .ok_or(DatabaseError::NotFound)
    }

    pub async fn create(&self, user: UserCreate) -> Result<User, DatabaseError> {
        sqlx::query_as(
            "
        INSERT INTO user (id, username) VALUES (?, ?)
        ON CONFLICT(id) DO UPDATE SET username = ?
        RETURNING id, username, about;
        ",
        )
        .bind(user.id)
        .bind(&user.username)
        .bind(&user.username)
        .fetch_optional(self.db)
        .await?
        .ok_or(DatabaseError::NotFound)
    }

    pub async fn patch(&self, user_id: u32, patch_user: UserPatch) -> Result<User, DatabaseError> {
        sqlx::query_as(
            "
        UPDATE user SET about = ? WHERE id = ?
        RETURNING id, username, about
        ",
        )
        .bind(patch_user.about)
        .bind(user_id)
        .fetch_optional(self.db)
        .await?
        .ok_or(DatabaseError::NotFound)
    }
}
