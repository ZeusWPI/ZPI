use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(FromRow, Serialize, Deserialize, PartialEq, Debug)]
pub struct Service {
    pub id: u32,
    pub name: String,
    pub api_key: String,
}

#[derive(Serialize, Deserialize)]
pub struct ServiceCreatePayload {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ServicePatchPayload {
    pub name: String,
}
