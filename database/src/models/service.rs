use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(FromRow, Serialize)]
pub struct Service {
    pub id: u32,
    pub name: String,
}
