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
