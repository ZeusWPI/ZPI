use std::{env, path::PathBuf, sync::LazyLock};

use fast_image_resize::{IntoImageView, Resizer, images::Image};
use image::{DynamicImage, GenericImageView, codecs::jpeg::JpegEncoder};
use tokio::{fs::File, io::AsyncWriteExt};

use crate::{error::AppError, format::SupportedFormat};

static IMAGE_PATH: LazyLock<String> =
    LazyLock::new(|| env::var("IMAGE_PATH").expect("IMAGE_PATH not present"));

pub struct ProfileImage {
    user_id: u32,
}

impl ProfileImage {
    pub fn new(user_id: u32) -> Self {
        Self { user_id }
    }

    pub fn with_data(self, data: &[u8]) -> Result<DataImage, AppError> {
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

    pub fn path(&self, size_opt: Option<u32>, format: SupportedFormat) -> PathBuf {
        let filename = match size_opt {
            Some(size) => format!("{}.{}.{}", self.user_id, size, format.extension()),
            None => format!("{}.{}", self.user_id, format.extension()),
        };
        PathBuf::from(IMAGE_PATH.to_string()).join(filename)
    }
}

pub struct DataImage {
    profile: ProfileImage,
    image: DynamicImage,
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
    pub async fn save_sizes(&self, sizes: &[u32]) -> Result<&Self, AppError> {
        for size in sizes {
            self.save_size(*size).await?;
        }

        Ok(self)
    }

    /// resize the image and save
    pub async fn save_size(&self, size: u32) -> Result<&Self, AppError> {
        // create a destination image buffer
        let mut dst_image = Image::new(
            size,
            size,
            self.image
                .pixel_type()
                .ok_or(AppError::Internal("image pixel type err".into()))?,
        );

        // resize image
        Resizer::new().resize(&self.image, &mut dst_image, None)?;

        let mut buffer = Vec::new();
        let (mut encoder, format) = (JpegEncoder::new(&mut buffer), SupportedFormat::Jpeg);

        // write resized image to buffer
        encoder.encode(dst_image.buffer(), size, size, self.image.color().into())?;

        // save image buffer to file
        let mut file = File::create(self.profile.path(Some(size), format)).await?;
        file.write_all(&buffer).await?;

        Ok(self)
    }
}

pub fn jpg_image_path(user_id: u32, size_opt: Option<u32>) -> PathBuf {
    let filename = match size_opt {
        Some(size) => format!("{user_id}.{size}.jpg"),
        None => format!("{user_id}.jpg"),
    };
    PathBuf::from(IMAGE_PATH.to_string()).join(filename)
}
