use axum::{Router, extract::DefaultBodyLimit, routing::get};
use database::Database;
use reqwest::StatusCode;
use tokio::fs;
use tower_http::{compression::CompressionLayer, cors::CorsLayer, trace::TraceLayer};
use tower_sessions::{MemoryStore, SessionManagerLayer, cookie::SameSite};

use crate::{
    error::AppError,
    handlers::{auth::AuthHandler, image::ImageHandler, user::UserHandler},
    image::IMAGE_PATH,
};

pub mod error;
pub mod handlers;
pub mod image;

pub async fn start_app() -> Result<(), AppError> {
    // create image directory
    if !IMAGE_PATH.exists() {
        fs::create_dir_all(image::IMAGE_PATH.as_path()).await?;
    }

    let db = Database::create_connect_migrate().await?;

    // setup layers
    let sess_store = MemoryStore::default();
    let sess_mw = SessionManagerLayer::new(sess_store).with_same_site(SameSite::Lax);
    let app = Router::new()
        .nest("/api", api_router())
        .layer(sess_mw)
        .layer(DefaultBodyLimit::max(10_485_760))
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::very_permissive())
        .with_state(db);

    // start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

pub fn api_router() -> Router<Database> {
    Router::new()
        .merge(AuthHandler::router())
        .nest("/image", ImageHandler::router())
        .nest("/users", UserHandler::router())
        .fallback(get(|| async { StatusCode::NOT_FOUND }))
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
