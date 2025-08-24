use std::{env, sync::LazyLock};

use axum::{
    Router,
    extract::DefaultBodyLimit,
    response::Html,
    routing::{get, post},
};
use error::AppError;
use pages::Page;
use tokio::io::{self};
use tower_http::{services::ServeDir, trace::TraceLayer};
use tower_sessions::{MemoryStore, Session, SessionManagerLayer, cookie::SameSite};

use crate::handlers::{
    auth::{Auth, ZauthUser},
    image::Image,
};

mod error;
mod format;
mod handlers;
mod image;
mod pages;

static LOG_LEVEL: LazyLock<String> =
    LazyLock::new(|| env::var("LOG_LEVEL").unwrap_or("INFO".into()));

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let _ = dotenvy::dotenv();

    let log_level = match LOG_LEVEL.as_str() {
        "DEBUG" => tracing::Level::DEBUG,
        "INFO" => tracing::Level::INFO,
        "WARN" => tracing::Level::WARN,
        _ => tracing::Level::INFO,
    };

    tracing_subscriber::fmt().with_max_level(log_level).init();

    let sess_store = MemoryStore::default();
    let sess_mw = SessionManagerLayer::new(sess_store).with_same_site(SameSite::Lax);
    let static_dir = ServeDir::new("./static");

    let app = Router::new()
        .route("/", get(index))
        .route("/login", get(Auth::login))
        .route("/oauth/callback", get(Auth::callback))
        .route("/logout", get(Auth::logout))
        .route("/image", post(Image::post).delete(Image::delete))
        .route("/image/{id}", get(Image::get))
        .nest_service("/static", static_dir)
        .route("/{*wildcard}", get(|| async { Page::error("404") }))
        .layer(sess_mw)
        .layer(DefaultBodyLimit::max(10_485_760))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

pub async fn index(session: Session) -> Result<Html<String>, AppError> {
    Ok(match session.get::<ZauthUser>("user").await? {
        None => Page::login(),
        Some(user) => Page::upload(&user.username, user.id),
    })
}
