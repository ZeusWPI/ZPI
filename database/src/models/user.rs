use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::models::tag::Tag;

#[derive(Debug, FromRow, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub about: String,
}

pub struct UserCreatePayload {
    pub id: u32,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPatchPayload {
    pub about: String,
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
