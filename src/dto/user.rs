use database::{
    Database,
    error::DatabaseError,
    models::{tag::Tag, user::UserPatch},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPatchPayload {
    pub about: String,
}

impl From<UserPatchPayload> for UserPatch {
    fn from(value: UserPatchPayload) -> Self {
        UserPatch { about: value.about }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct UserProfile {
    pub id: u32,
    pub username: String,
    pub about: String,
    pub tags: Vec<Tag>,
}

pub enum UserId {
    Username(String),
    Id(u32),
}

impl From<String> for UserId {
    fn from(value: String) -> Self {
        match value.parse::<u32>() {
            Ok(id) => Self::Id(id),
            Err(_) => Self::Username(value),
        }
    }
}

impl From<u32> for UserId {
    fn from(value: u32) -> Self {
        Self::Id(value)
    }
}

impl UserProfile {
    pub async fn get(db: &Database, user_id: UserId) -> Result<UserProfile, DatabaseError> {
        let user = match user_id {
            UserId::Username(username) => db.users().by_username(username).await?,
            UserId::Id(id) => db.users().by_id(id).await?,
        };
        let tags = db.tags().for_user(user.id).await?;

        Ok(UserProfile {
            id: user.id,
            username: user.username,
            about: user.about,
            tags,
        })
    }
}
