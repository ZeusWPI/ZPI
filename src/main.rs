use std::{env, sync::LazyLock};

use axum::{
    Router,
    body::Body,
    extract::{DefaultBodyLimit, Multipart, Path, Query},
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
};
use error::AppError;
use image::ZPIImage;
use pages::Page;
use rand::distr::{Alphanumeric, SampleString};
use serde::{Deserialize, Serialize};
use tokio_util::io::ReaderStream;
use tower_http::trace::TraceLayer;
use tower_sessions::{MemoryStore, Session, SessionManagerLayer, cookie::SameSite};

use crate::image::image_path;

mod error;
mod image;
mod pages;

static ZAUTH_URL: LazyLock<String> =
    LazyLock::new(|| env::var("ZAUTH_URL").expect("ZAUTH_URL not present"));
static CALLBACK_URL: LazyLock<String> =
    LazyLock::new(|| env::var("ZAUTH_CALLBACK_PATH").expect("ZAUTH_CALLBACK_PATH not present"));
static ZAUTH_CLIENT_ID: LazyLock<String> =
    LazyLock::new(|| env::var("ZAUTH_CLIENT_ID").expect("ZAUTH_CLIENT_ID not present"));
static ZAUTH_CLIENT_SECRET: LazyLock<String> =
    LazyLock::new(|| env::var("ZAUTH_CLIENT_SECRET").expect("ZAUTH_CLIENT_SECRET not present"));
static LOG_LEVEL: LazyLock<String> =
    LazyLock::new(|| env::var("LOG_LEVEL").expect("LOG_LEVEL not present"));

static PLACEHOLDER: &[u8] = include_bytes!("../static/placeholder.jpg");

#[tokio::main]
async fn main() {
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

    let app = Router::new()
        .route("/", get(index))
        .route("/login", get(login))
        .route("/oauth/callback", get(callback))
        .route("/logout", get(logout))
        .route("/image", post(post_image))
        .route("/image/{id}", get(get_image))
        .route("/{*wildcard}", get(|| async { Page::error("404") }))
        .layer(sess_mw)
        .layer(DefaultBodyLimit::max(10_485_760))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub async fn index(session: Session) -> Result<Html<String>, AppError> {
    Ok(match session.get::<ZauthUser>("user").await? {
        None => Page::login(),
        Some(user) => Page::upload(&user.username, user.id),
    })
}

pub async fn post_image(session: Session, mut multipart: Multipart) -> Result<Redirect, AppError> {
    match session.get::<ZauthUser>("user").await? {
        None => Ok(Redirect::to("/")),
        Some(user) => {
            while let Some(field) = multipart.next_field().await? {
                if let Some("image_file") = field.name() {
                    let data = field.bytes().await?;
                    match infer::get(&data)
                        .ok_or(AppError::WrongFileType)?
                        .mime_type()
                    {
                        "image/jpeg" => {}
                        _ => return Err(AppError::WrongFileType),
                    }

                    let image = ZPIImage::from_data(&data, user.id);
                    image.save_multiple_resized(&[64, 128, 256, 512]).await?;
                    image.save_original().await?;
                    return Ok(Redirect::to("/"));
                }
            }
            Err(AppError::NoFile)
        }
    }
}

#[derive(Deserialize)]
pub struct PlaceholderQuery {
    placeholder: Option<bool>,
    size: Option<u32>,
}

pub async fn get_image(
    Query(params): Query<PlaceholderQuery>,
    Path(id): Path<u32>,
) -> Result<Body, AppError> {
    let path = image_path(id, params.size);
    let file = tokio::fs::File::open(path).await;
    match file {
        Err(_) => match params.placeholder {
            Some(true) => Ok(Body::from(PLACEHOLDER)),
            _ => Err(AppError::ImageNotFound),
        },
        Ok(file) => Ok(Body::from_stream(ReaderStream::new(file))),
    }
}
