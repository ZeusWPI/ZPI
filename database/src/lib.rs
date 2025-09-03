use std::{env, sync::LazyLock};

use sqlx::{SqlitePool, migrate::MigrateDatabase, sqlite::SqlitePoolOptions};

use crate::{
    error::DatabaseError,
    repos::{tag::TagRepo, user::UserRepo},
};

pub mod models {
    pub mod tag;
    pub mod user;
}

pub mod repos {
    pub mod tag;
    pub mod user;
}

pub mod error;

pub static DATABASE_URL: LazyLock<String> =
    LazyLock::new(|| env::var("DATABASE_URL").expect("DATABASE_URL not present"));

#[derive(Clone)]
pub struct Database {
    db: SqlitePool,
}

impl Database {
    pub async fn new(db: SqlitePool) -> Self {
        Self { db }
    }

    pub async fn create_connect_migrate() -> Result<Self, DatabaseError> {
        // create database if not exists
        sqlx::Sqlite::create_database(&DATABASE_URL).await?;

        // conntect to database
        let db = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&DATABASE_URL)
            .await?;

        // run migrations
        sqlx::migrate!("../migrations").run(&db).await?;

        Ok(Self { db })
    }

    pub fn users<'a>(&'a self) -> UserRepo<'a> {
        UserRepo::new(&self.db)
    }

    pub fn tags<'a>(&'a self) -> TagRepo<'a> {
        TagRepo::new(&self.db)
    }
}
