use serde::{Deserialize, Serialize};
use sqlx::{Decode, FromRow, SqlitePool};

use crate::{
    error::AppError,
    handlers::{AuthenticatedUser, auth::ZauthUser},
    models::tag::Tag,
};

#[derive(Debug, FromRow, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub about: String,
}

impl User {
    pub fn new(id: u32, username: String, about: String) -> Self {
        User {
            id,
            username,
            about,
        }
    }

    pub async fn get_by_id(db: &SqlitePool, id: u32) -> Result<Self, AppError> {
        Ok(
            sqlx::query_as("SELECT id, username, about FROM user WHERE id == ? LIMIT 1;")
                .bind(id)
                .fetch_one(db)
                .await?,
        )
    }

    pub async fn get_by_username(db: &SqlitePool, username: String) -> Result<Self, AppError> {
        Ok(
            sqlx::query_as("SELECT id, username, about FROM user WHERE username == ? LIMIT 1;")
                .bind(username)
                .fetch_one(db)
                .await?,
        )
    }

    pub async fn create(&self, db: &SqlitePool) -> Result<(), AppError> {
        sqlx::query(
            "
            INSERT INTO user (id, username, about) VALUES (?, ?, ?)
            ON CONFLICT(id) DO UPDATE SET username = ?;
            ",
        )
        .bind(self.id)
        .bind(&self.username)
        .bind(&self.about)
        .bind(&self.username)
        .execute(db)
        .await?;
        Ok(())
    }
}

impl From<ZauthUser> for User {
    fn from(value: ZauthUser) -> Self {
        Self::new(value.id, value.username, String::new())
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserPatchPayload {
    pub about: String,
}

impl UserPatchPayload {
    pub async fn update_user(
        self,
        db: &SqlitePool,
        user: AuthenticatedUser,
    ) -> Result<Option<User>, AppError> {
        Ok(sqlx::query_as(
            "
            UPDATE user SET about = ? WHERE id = ?
            RETURNING id, username, about
            ",
        )
        .bind(self.about)
        .bind(user.id)
        .fetch_optional(db)
        .await?)
    }
}

#[derive(Debug, FromRow, Decode, Serialize, Deserialize, PartialEq)]
pub struct UserProfilePayload {
    pub id: u32,
    pub username: String,
    pub about: String,
    pub tags: Vec<Tag>,
}

impl UserProfilePayload {
    pub async fn get_by_id(db: &SqlitePool, id: u32) -> Result<Self, AppError> {
        let user = dbg!(User::get_by_id(db, id).await)?;
        let tags = dbg!(Tag::for_user(db, id).await)?;

        Ok(UserProfilePayload {
            id: user.id,
            username: user.username,
            about: user.about,
            tags,
        })
    }

    pub async fn get_by_username(db: &SqlitePool, username: String) -> Result<Self, AppError> {
        let user = dbg!(User::get_by_username(db, username).await)?;
        let tags = dbg!(Tag::for_user(db, user.id).await)?;

        Ok(UserProfilePayload {
            id: user.id,
            username: user.username,
            about: user.about,
            tags,
        })
    }
}
