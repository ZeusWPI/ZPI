use std::{
    env,
    hash::{DefaultHasher, Hash, Hasher},
    path,
    sync::LazyLock,
};

use auth::ZauthUser;
use axum::{
    Router,
    body::Body,
    extract::{DefaultBodyLimit, Multipart, Path, Query},
    http::HeaderValue,
    response::{Html, IntoResponse, Redirect, Response},
    routing::{get, post},
};
use axum_extra::{TypedHeader, headers::IfNoneMatch};
use error::AppError;
use headers::ETag;
use image::ProfileImage;
use pages::Page;
use reqwest::{
    StatusCode,
    header::{CONTENT_TYPE, ETAG},
};
use serde::Deserialize;
use tokio::io::{self, ErrorKind::NotFound};
use tokio_util::io::ReaderStream;
use tower_http::trace::TraceLayer;
use tower_sessions::{MemoryStore, Session, SessionManagerLayer, cookie::SameSite};

use crate::image::jpg_image_path;

mod auth;
mod error;
mod format;
mod image;
mod pages;

static LOG_LEVEL: LazyLock<String> =
    LazyLock::new(|| env::var("LOG_LEVEL").unwrap_or("INFO".into()));

static PLACEHOLDER: &[u8] = include_bytes!("../static/placeholder.jpg");

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

    let app = Router::new()
        .route("/", get(index))
        .route("/login", get(auth::login))
        .route("/oauth/callback", get(auth::callback))
        .route("/logout", get(auth::logout))
        .route("/image", post(post_image))
        .route("/image/{id}", get(get_image))
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

pub async fn post_image(session: Session, mut multipart: Multipart) -> Result<Redirect, AppError> {
    match session.get::<ZauthUser>("user").await? {
        None => Ok(Redirect::to("/")),
        Some(user) => {
            while let Some(field) = multipart.next_field().await? {
                if let Some("image_file") = field.name() {
                    let data = field.bytes().await?;

                    ProfileImage::new(user.id)
                        .with_data(&data)?
                        .cropped()
                        .save_sizes(&[64, 128, 256, 512])
                        .await?;

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
    if_none_match: Option<TypedHeader<IfNoneMatch>>,
) -> Result<Response, AppError> {
    // default size
    let size = params.size.unwrap_or(256);
    let path = jpg_image_path(id, Some(size));

    let etag_opt = file_modified_etag(&path).await?;

    // return early if etag matches
    if let Some(if_none_match) = &if_none_match
        && let Some(etag_string) = &etag_opt
        && let Ok(etag) = etag_string.parse::<ETag>()
        && !if_none_match.precondition_passes(&etag)
    {
        return Ok(StatusCode::NOT_MODIFIED.into_response());
    }

    // get image (or placeholder, if requested) from disk
    let file = tokio::fs::File::open(&path).await;
    let mut resp = match file {
        Ok(file) => Body::from_stream(ReaderStream::new(file)),
        Err(e) if e.kind() == NotFound => match params.placeholder {
            Some(true) => Body::from(PLACEHOLDER),
            _ => return Err(AppError::ImageNotFound),
        },
        Err(e) => return Err(e.into()),
    }
    .into_response();

    // set etag header if possible
    if let Some(etag_string) = etag_opt
        && let Ok(etag_hval) = etag_string.parse()
    {
        resp.headers_mut().insert(ETAG, etag_hval);
    }

    resp.headers_mut()
        .insert(CONTENT_TYPE, HeaderValue::from_static("image/jpeg"));

    Ok(resp)
}

async fn file_modified_etag(path: &path::Path) -> Result<Option<String>, AppError> {
    let metadata = match tokio::fs::metadata(&path).await {
        Ok(metadata) => metadata,
        Err(err) if err.kind() == NotFound => return Ok(None),
        err => err?,
    };

    let modified = metadata.modified()?;

    let mut hasher = DefaultHasher::new();
    modified.hash(&mut hasher);
    let hash = hasher.finish().to_string();

    Ok(Some(format!("\"{hash}\"")))
}
