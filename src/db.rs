use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{FromRow, SqlitePool};
use std::env;
use std::sync::LazyLock;

static DB_LOCATION: LazyLock<String> =
    LazyLock::new(|| env::var("DB_LOCATION").expect("DB_LOCATION not present"));

pub async fn create_client() -> SqlitePool {
    dbg!(DB_LOCATION.as_str());
    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&format!("sqlite://{}", DB_LOCATION.as_str()))
        .await
        .unwrap()
}

