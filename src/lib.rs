use axum::{
    Router,
    extract::DefaultBodyLimit,
    middleware::from_extractor,
    routing::{get, post},
};
use database::Database;
use reqwest::StatusCode;
use tokio::fs;
use tower_http::{compression::CompressionLayer, cors::CorsLayer, trace::TraceLayer};
use tower_sessions::{MemoryStore, SessionManagerLayer, cookie::SameSite};

use crate::{
    config::AppConfig,
    error::AppError,
    extractors::{Admin, AuthenticatedUser},
    handlers::{
        achievement::AchievementHandler, auth::AuthHandler, image::ImageHandler,
        service::ServiceHandler, user::UserHandler, version::VersionHandler,
    },
};

pub mod config;
pub mod error;
pub mod extractors;
pub mod handlers;
pub mod image;

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub config: AppConfig,
}

pub async fn start_app(config: AppConfig) -> Result<(), AppError> {
    // create image directory
    if !config.image_path.exists() {
        fs::create_dir_all(config.image_path.as_path()).await?;
    }

    let db = Database::create_connect_migrate(&config.database_url).await?;

    let state = AppState { db, config };

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
        .with_state(state);

    // start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

pub fn api_router() -> Router<AppState> {
    Router::new()
        .merge(open_routes())
        .merge(authenticated_routes())
        .merge(admin_routes())
        .fallback(get(|| async { StatusCode::NOT_FOUND }))
}

fn open_routes() -> Router<AppState> {
    Router::new()
        .route("/login", get(AuthHandler::login))
        .route("/oauth/callback", get(AuthHandler::callback))
        .route("/image/{id}", get(ImageHandler::get))
        .route("/version", get(VersionHandler::get))
}

fn authenticated_routes() -> Router<AppState> {
    Router::new()
        .nest("/users", UserHandler::router())
        .route("/logout", get(AuthHandler::logout))
        .route(
            "/image",
            post(ImageHandler::post).delete(ImageHandler::delete),
        )
        .route_layer(from_extractor::<AuthenticatedUser>())
}

fn admin_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/services",
            get(ServiceHandler::get).post(ServiceHandler::post),
        )
        .route(
            "/services/{id}/achievements",
            get(AchievementHandler::get_for_service),
        )
        .route_layer(from_extractor::<Admin>())
}

#[allow(clippy::expect_used)]
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
