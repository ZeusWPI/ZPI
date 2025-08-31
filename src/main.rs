use std::error::Error;

use axum::{Router, extract::DefaultBodyLimit, response::Html, routing::get};
use error::AppError;
use pages::Page;
use reqwest::StatusCode;
use sqlx::{SqlitePool, migrate::MigrateDatabase};
use tokio::{
    fs,
    io::{self},
};
use tower_http::{
    compression::CompressionLayer, cors::CorsLayer, services::ServeDir, trace::TraceLayer,
};
use tower_sessions::{MemoryStore, Session, SessionManagerLayer, cookie::SameSite};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

use crate::{
    handlers::{
        auth::{AuthHandler, ZauthUser},
        image::ImageHandler,
        user::UserHandler,
    },
    image::IMAGE_PATH,
};

mod db;
mod error;
mod format;
mod handlers;
mod image;
mod models;
mod pages;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = dotenvy::dotenv();

    // setup logging
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_env("LOG_LEVEL"))
        .init();

    // create image directory
    if !IMAGE_PATH.exists() {
        fs::create_dir_all(image::IMAGE_PATH.as_path()).await?;
    }

    // setup database
    sqlx::Sqlite::create_database(&db::DATABASE_URL).await?;
    let db = db::create_conn().await;
    sqlx::migrate!().run(&db).await?;

    // setup layers
    let sess_store = MemoryStore::default();
    let sess_mw = SessionManagerLayer::new(sess_store).with_same_site(SameSite::Lax);
    let app = create_router()
        .layer(sess_mw)
        .layer(DefaultBodyLimit::max(10_485_760))
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(db);

    // start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

pub fn create_router() -> Router<SqlitePool> {
    let static_dir = ServeDir::new("./static");

    Router::new()
        .route("/", get(index))
        .merge(AuthHandler::router())
        .nest("/image", ImageHandler::router())
        .nest("/users", UserHandler::router())
        .nest_service("/static", static_dir)
        .fallback(get(|| async {
            (
                StatusCode::NOT_FOUND,
                Page::error(StatusCode::NOT_FOUND, "404"),
            )
        }))
}

pub async fn index(session: Session) -> Result<Html<String>, AppError> {
    Ok(match session.get::<ZauthUser>("user").await? {
        None => Page::login(),
        Some(user) => Page::upload(&user.username, user.id),
    })
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
