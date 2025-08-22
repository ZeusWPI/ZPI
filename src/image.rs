use std::{
    env,
    io::ErrorKind,
    path::PathBuf,
    process::ExitStatus,
    sync::{Arc, LazyLock},
};

use axum::{
    body::Body,
    http::HeaderValue,
    response::{IntoResponse, Response},
};

use reqwest::header::CONTENT_TYPE;
use tokio::{fs::File, process::Command, task::JoinSet};
use tokio_util::io::ReaderStream;

use crate::{PLACEHOLDER, error::AppError, format::SupportedFormat};

static IMAGE_PATH: LazyLock<String> =
    LazyLock::new(|| env::var("IMAGE_PATH").expect("IMAGE_PATH not present"));

static MAGICK_PATH: LazyLock<String> =
    LazyLock::new(|| env::var("MAGICK_PATH").expect("MAGICK_PATH not present"));

static IMAGE_SAVE_TYPE: SupportedFormat = SupportedFormat::Webp;

pub struct ProfileImage {
    user_id: u32,
}

pub struct DataImage {
    profile: ProfileImage,
}

pub enum ResponseImage {
    File(File),
    Placeholder,
}

impl ProfileImage {
    pub fn new(user_id: u32) -> Self {
        Self { user_id }
    }

    pub async fn with_data(self, data: &[u8]) -> Result<DataImage, AppError> {
        // save original
        let path = self.path_orig();
        tokio::fs::write(path, data).await?;

        Ok(DataImage { profile: self })
    }

    pub async fn get(&self, size: u32) -> Result<ResponseImage, AppError> {
        let file = tokio::fs::File::open(&self.path(size))
            .await
            .map_err(|err| match err.kind() {
                ErrorKind::NotFound => AppError::ImageNotFound,
                _ => err.into(),
            })?;

        Ok(ResponseImage::File(file))
    }

    pub async fn get_with_placeholder(&self, size: u32) -> Result<ResponseImage, AppError> {
        match self.get(size).await {
            Err(AppError::ImageNotFound) => Ok(ResponseImage::Placeholder),
            other => other,
        }
    }

    pub fn path_orig(&self) -> PathBuf {
        PathBuf::from(IMAGE_PATH.to_string()).join(self.user_id.to_string())
    }

    pub fn path(&self, size: u32) -> PathBuf {
        let filename = format!("{}.{}.{}", self.user_id, size, IMAGE_SAVE_TYPE.extension());
        PathBuf::from(IMAGE_PATH.to_string()).join(filename)
    }
}

impl DataImage {
    /// save as multiple sizes
    pub async fn save_sizes(self, sizes: &[u32]) -> Result<(), AppError> {
        let mut set = JoinSet::new();
        let image = Arc::new(self);

        for &size in sizes {
            let image_arc = image.clone();
            set.spawn(async move { image_arc.save_size(size).await });
        }

        while let Some(res) = set.join_next().await {
            res.unwrap()?;
        }

        Ok(())
    }

    /// resize the image and save
    pub async fn save_size(&self, size: u32) -> Result<(), AppError> {
        // magick 102 -coalesce -resize "64x64^" -gravity center -crop "64x64+0+0" +repage out.webp
        let output = Command::new("")
            .args([
                self.profile
                    .path_orig()
                    .to_str()
                    .ok_or(AppError::Internal("invalid path".into()))?,
                "-coalesce",
                "-filter",
                "Robidoux",
                "-resize",
                format!("{size}x{size}^").as_str(),
                "-gravity",
                "center",
                "-crop",
                format!("{size}x{size}+0+0").as_str(),
                "+repage",
                self.profile
                    .path(size)
                    .to_str()
                    .ok_or(AppError::Internal("invalid path".into()))?,
            ])
            .output()
            .await?;

        // if magick was not success
        if !output.status.success() {
            return Err(AppError::Magick(
                str::from_utf8(&output.stderr)
                    .or(Err(AppError::Internal("utf8".into())))?
                    .to_string(),
            ));
        }

        Ok(())
    }
}

impl IntoResponse for ResponseImage {
    fn into_response(self) -> Response {
        let mut resp = match self {
            Self::Placeholder => Body::from(PLACEHOLDER),
            Self::File(file) => Body::from_stream(ReaderStream::new(file)),
        }
        .into_response();

        resp.headers_mut().insert(
            CONTENT_TYPE,
            HeaderValue::from_static(IMAGE_SAVE_TYPE.mime_type()),
        );

        resp
    }
}
