use std::{
    env,
    io::ErrorKind,
    path::PathBuf,
    sync::{Arc, LazyLock},
};

use axum::{
    body::Body,
    http::HeaderValue,
    response::{IntoResponse, Response},
};
use fast_image_resize::{IntoImageView, Resizer, images::Image};
use image::{DynamicImage, GenericImageView};
use reqwest::header::CONTENT_TYPE;
use tokio::{fs::File, io::AsyncWriteExt, task::JoinSet};
use tokio_util::io::ReaderStream;
use webp::Encoder;

use crate::{PLACEHOLDER, error::AppError, format::SupportedFormat};

static IMAGE_PATH: LazyLock<String> =
    LazyLock::new(|| env::var("IMAGE_PATH").expect("IMAGE_PATH not present"));

static IMAGE_SAVE_TYPE: SupportedFormat = SupportedFormat::Webp;

pub struct ProfileImage {
    user_id: u32,
}

pub struct DataImage {
    profile: ProfileImage,
    image: DynamicImage,
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

        let format = SupportedFormat::guess(data)?;
        let image = image::load_from_memory_with_format(data, format.into())?;

        if image.dimensions() > (10_000, 10_000) {
            return Err(AppError::ImageResTooLarge);
        }

        Ok(DataImage {
            profile: self,
            image,
        })
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
    /// crop the image to a square
    pub fn cropped(self) -> Self {
        let (width, heigth) = self.image.dimensions();
        let crop_dimension = width.min(heigth);
        let cropped_img = self.image.crop_imm(
            (width - crop_dimension) / 2,
            (heigth - crop_dimension) / 2,
            crop_dimension,
            crop_dimension,
        );

        Self {
            image: cropped_img,
            ..self
        }
    }

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
        // create a destination image buffer
        let mut resized_image = Image::new(
            size,
            size,
            self.image
                .pixel_type()
                .ok_or(AppError::Internal("image pixel type err".into()))?,
        );

        // resize image
        Resizer::new().resize(&self.image, &mut resized_image, None)?;

        let encoder = Encoder::from_rgb(resized_image.buffer(), size, size);
        let webp = encoder.encode(85.).to_vec();

        // save image buffer to file
        let mut file = File::create(self.profile.path(size)).await?;
        file.write_all(&webp).await?;

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
