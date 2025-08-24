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

use rand::{SeedableRng, rngs::SmallRng, seq::IndexedRandom};
use reqwest::header::CONTENT_TYPE;
use svg::{
    Document,
    node::element::{Definitions, Group, Mask, Polygon, Polyline, Rectangle, Use},
};
use tokio::{fs::File, process::Command, task::JoinSet};
use tokio_util::io::ReaderStream;

use crate::{error::AppError, format::SupportedFormat};

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
    Placeholder(u32),
}

impl ProfileImage {
    pub fn new(user_id: u32) -> Self {
        tracing::debug!("new user with id {user_id}");
        Self { user_id }
    }

    pub async fn with_data(self, data: &[u8]) -> Result<DataImage, AppError> {
        // save original
        tracing::debug!(
            "saving original image on {} with data length {}",
            self.path_orig().display(),
            data.len()
        );
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
            Err(AppError::ImageNotFound) => Ok(ResponseImage::Placeholder(self.user_id)),
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
        tracing::debug!("saving images with sizes {:?}", sizes);
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
        let orig_path = self.profile.path_orig();
        let sized_path = self.profile.path(size);
        let resize_arg = format!("{size}x{size}^");
        let crop_arg = format!("{size}x{size}+0+0");

        let args = [
            orig_path
                .to_str()
                .ok_or(AppError::Internal("invalid path".into()))?,
            "-coalesce",
            "-filter",
            "Robidoux",
            "-resize",
            resize_arg.as_str(),
            "-gravity",
            "center",
            "-crop",
            crop_arg.as_str(),
            "+repage",
            sized_path
                .to_str()
                .ok_or(AppError::Internal("invalid path".into()))?,
        ];

        tracing::debug!(
            "running command {} with args {:?}",
            MAGICK_PATH.as_str(),
            args
        );

        let output = Command::new(MAGICK_PATH.as_str())
            .args(args)
            .output()
            .await?;

        tracing::debug!("command ran with status code {}", output.status);
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
        match self {
            Self::Placeholder(user_id) => {
                let mut body = Body::from(make_placeholder(user_id)).into_response();
                body.headers_mut()
                    .insert(CONTENT_TYPE, HeaderValue::from_static("image/svg+xml"));
                body
            }
            Self::File(file) => {
                let mut body = Body::from_stream(ReaderStream::new(file)).into_response();
                body.headers_mut().insert(
                    CONTENT_TYPE,
                    HeaderValue::from_static(IMAGE_SAVE_TYPE.mime_type()),
                );
                body
            }
        }
    }
}

fn make_placeholder(user_id: u32) -> Vec<u8> {
    // svg made by @flynn
    let mut rand_gen = SmallRng::seed_from_u64(user_id as u64);
    let polygon_points = "9.0,0.0 4.5,7.794 -4.5,7.794 -9.0,0 -4.5,-7.794 4.5,-7.794";

    let mask_polygon = Polygon::new()
        .set("points", polygon_points)
        .set("fill", "white");

    let mask = Mask::new()
        .set("id", "poly")
        .set("mask-type", "luminance")
        .set("x", -100)
        .set("y", -100)
        .set("width", 200)
        .set("height", 200)
        .set("maskUnits", "userSpaceOnUse")
        .add(mask_polygon);

    let cube_polygon = Polygon::new().set("points", polygon_points);

    let cube_polyline = Polyline::new()
        .set("mask", "url(#poly)")
        .set("points", "9,0 0,0 -4.5,7.794 0,0 -4.5,-7.794")
        .set("style", "fill:none;stroke:#02020244;stroke-width:.6");

    let cube_group = Group::new()
        .set("id", "cube")
        .add(cube_polygon)
        .add(cube_polyline);

    let defs = Definitions::new().add(cube_group);

    // TODO randomize rotation
    let mut main_group = Group::new().set("transform", "rotate(0 32 32)");

    let use_data = vec![
        (22.0, 14.68),
        (42.0, 14.68),
        (12.0, 32.0),
        (52.0, 32.0),
        (22.0, 49.32),
        (42.0, 49.32),
    ];

    let colors = ["#FFBE0B", "#FF4037", "#FF006E", "#8338EC", "#3A86FF"];

    for (x, y) in use_data {
        let use_element = Use::new()
            .set("xlink:href", "#cube")
            .set("x", x)
            .set("y", y)
            .set("fill", *colors.choose(&mut rand_gen).unwrap());
        main_group = main_group.add(use_element);
    }

    // make the middle one zeus orange
    let use_element = Use::new()
        .set("xlink:href", "#cube")
        .set("x", 32)
        .set("y", 32)
        .set("fill", "#FF7F00");
    main_group = main_group.add(use_element);

    let background = Rectangle::new()
        .set("width", "64")
        .set("height", "64")
        .set("x", "0")
        .set("y", "0")
        .set("fill", "#EEE");

    let document = Document::new()
        .set("viewBox", "0 0 64 64")
        .set("xmlns:xlink", "http://www.w3.org/1999/xlink")
        .set("id", "flynn")
        .add(background)
        .add(mask)
        .add(defs)
        .add(main_group);

    let mut buffer = Vec::new();
    svg::write(&mut buffer, &document).unwrap();

    buffer
}
