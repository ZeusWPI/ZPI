use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub about: String,
}

pub struct UserCreate {
    pub id: u32,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPatch {
    pub about: String,
}
