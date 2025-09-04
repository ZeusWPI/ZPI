use std::{
    hash::{DefaultHasher, Hash, Hasher},
    io::ErrorKind::NotFound,
    path,
};

use axum::{
    Router,
    body::{Body, Bytes, to_bytes},
    extract::{Path, Query},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use axum_extra::TypedHeader;
use headers::{ETag, IfNoneMatch};
use reqwest::{StatusCode, header::ETAG};
use serde::Deserialize;

use crate::{
    AppState, config::AppConfig, error::AppError, handlers::AuthenticatedUser, image::ProfileImage,
};

static SIZES: &[u32] = &[64, 128, 256, 512];
static MAX_SIZE: u32 = 512;

pub struct ImageHandler;

impl ImageHandler {
    pub fn router() -> Router<AppState> {
        Router::new()
            .route("/", post(Self::post).delete(Self::delete))
            .route("/{id}", get(Self::get))
    }

    pub async fn get(
        Query(params): Query<GetImageQuery>,
        Path(user_id): Path<u32>,
        if_none_match: Option<TypedHeader<IfNoneMatch>>,
        config: AppConfig,
    ) -> Result<Response, AppError> {
        // default size
        let requested_size = params.size.unwrap_or(256);
        // get closest size that is bigger, or largest if none are bigger
        let size = *SIZES
            .iter()
            .filter(|x| **x > requested_size)
            .min()
            .unwrap_or(&MAX_SIZE);

        let profile = ProfileImage::new(user_id, config);
        let etag_opt = file_modified_etag(&profile.path(size)).await?;

        // return early if etag matches
        if etag_matches(&if_none_match, &etag_opt) {
            return Ok(StatusCode::NOT_MODIFIED.into_response());
        }

        // get image (or placeholder, if requested) from disk
        let mut resp = match params.placeholder {
            Some(false) => profile.get(size).await,
            _ => profile.get_with_placeholder(size).await,
        }?
        .into_response();

        // set etag header if possible
        if let Some(etag_string) = etag_opt
            && let Ok(etag_header_val) = etag_string.parse()
        {
            resp.headers_mut().insert(ETAG, etag_header_val);
        }

        Ok(resp)
    }

    pub async fn post(
        user: AuthenticatedUser,
        config: AppConfig,
        body: Body,
    ) -> Result<StatusCode, AppError> {
        let data: Bytes = to_bytes(body, usize::MAX).await?;

        ProfileImage::new(user.id, config)
            .with_data(&data)
            .await?
            .save_sizes(SIZES)
            .await?;

        Ok(StatusCode::NO_CONTENT)
    }

    pub async fn delete(
        user: AuthenticatedUser,
        config: AppConfig,
    ) -> Result<StatusCode, AppError> {
        let profile = ProfileImage::new(user.id, config);
        for size in SIZES {
            if let Err(e) = tokio::fs::remove_file(profile.path(*size)).await
                && e.kind() != NotFound
            {
                Err(e)?;
            }
        }
        tokio::fs::remove_file(profile.path_orig()).await?;
        Ok(StatusCode::NO_CONTENT)
    }
}

#[derive(Deserialize)]
pub struct GetImageQuery {
    placeholder: Option<bool>,
    size: Option<u32>,
}

fn etag_matches(header: &Option<TypedHeader<IfNoneMatch>>, etag_string: &Option<String>) -> bool {
    if let Some(if_none_match) = header
        && let Some(etag_string) = etag_string
        && let Ok(etag) = etag_string.parse::<ETag>()
        && !if_none_match.precondition_passes(&etag)
    {
        true
    } else {
        false
    }
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
