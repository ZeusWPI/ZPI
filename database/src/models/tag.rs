use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize, PartialEq)]
pub struct Tag {
    id: u32,
    name: String,
    r#type: String,
    description: Option<String>,
}
