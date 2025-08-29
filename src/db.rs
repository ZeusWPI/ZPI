use sqlx::SqlitePool;
use sqlx::sqlite::SqlitePoolOptions;
use std::env;
use std::sync::LazyLock;

static DATABASE_URL: LazyLock<String> =
    LazyLock::new(|| env::var("DATABASE_URL").expect("DATABASE_URL not present"));

pub async fn create_conn() -> SqlitePool {
    dbg!(DATABASE_URL.as_str());
    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&DATABASE_URL)
        .await
        .unwrap()
}
