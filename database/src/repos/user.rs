use sqlx::SqlitePool;

use crate::{
    error::DatabaseError,
    models::user::{User, UserCreatePayload, UserPatchPayload, UserProfile},
    repos::tag::TagRepo,
};

pub enum UserId {
    Username(String),
    Id(u32),
}

impl UserId {
    pub fn new(user_id_or_name: String) -> Self {
        match user_id_or_name.parse::<u32>() {
            Ok(id) => Self::Id(id),
            Err(_) => Self::Username(user_id_or_name),
        }
    }
}

pub struct UserRepo<'a> {
    db: &'a SqlitePool,
}

impl<'a> UserRepo<'a> {
    pub fn new(db: &'a SqlitePool) -> Self {
        Self { db }
    }

    pub(crate) async fn by_id(&self, id: u32) -> Result<User, DatabaseError> {
        sqlx::query_as("SELECT id, username, about FROM user WHERE id == ? LIMIT 1;")
            .bind(id)
            .fetch_optional(self.db)
            .await?
            .ok_or(DatabaseError::NotFound)
    }

    pub(crate) async fn by_username(&self, username: String) -> Result<User, DatabaseError> {
        sqlx::query_as("SELECT id, username, about FROM user WHERE username == ? LIMIT 1;")
            .bind(username)
            .fetch_optional(self.db)
            .await?
            .ok_or(DatabaseError::NotFound)
    }

    pub async fn create(&self, user: UserCreatePayload) -> Result<User, DatabaseError> {
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

    pub async fn profile(&self, user_id: UserId) -> Result<UserProfile, DatabaseError> {
        let user = match user_id {
            UserId::Username(username) => self.by_username(username).await?,
            UserId::Id(id) => self.by_id(id).await?,
        };
        let tags = TagRepo::new(self.db).for_user(user.id).await?;

        Ok(UserProfile {
            id: user.id,
            username: user.username,
            about: user.about,
            tags,
        })
    }

    pub async fn patch(
        &self,
        user_id: u32,
        patch_user: UserPatchPayload,
    ) -> Result<User, DatabaseError> {
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
