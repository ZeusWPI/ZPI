use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize, PartialEq, JsonSchema)]
pub struct Tag {
    pub id: u32,
    pub name: String,
    pub category: String,
    pub description: Option<String>,
}
