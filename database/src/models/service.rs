use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(FromRow, Serialize, Deserialize, PartialEq, Debug)]
pub struct Service {
    pub id: u32,
    pub name: String,
}
