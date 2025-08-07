use std::{env, path::PathBuf, sync::LazyLock};

use fast_image_resize::{IntoImageView, Resizer, images::Image};
use image::codecs::jpeg::JpegEncoder;
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

use crate::error::AppError;

static IMAGE_PATH: LazyLock<String> =
    LazyLock::new(|| env::var("IMAGE_PATH").expect("IMAGE_PATH not present"));

pub struct ZPIImage<'a> {
    data: &'a [u8],
    user_id: u32,
}

impl<'a> ZPIImage<'a> {
    pub fn from_data(data: &'a [u8], user_id: u32) -> Self {
        ZPIImage { data, user_id }
    }

    pub async fn save_original(&self) -> Result<(), AppError> {
        fs::write(image_path(self.user_id, None), self.data).await?;
        Ok(())
    }

    pub async fn save_multiple_resized(&self, sizes: &[u32]) -> Result<(), AppError> {
        for size in sizes {
            self.save_resized(*size as u32).await?;
        }

        Ok(())
    }

    pub async fn save_resized(&self, size: u32) -> Result<(), AppError> {
        // load image from memory data
        let src_image = image::load_from_memory_with_format(self.data, image::ImageFormat::Jpeg)?;

        // create a destination image buffer
        let mut dst_image = Image::new(
            size,
            size,
            src_image
                .pixel_type()
                .ok_or(AppError::Internal("image pixel type errr".into()))?,
        );

        // resize image
        // TODO cut
        let mut resizer = Resizer::new();
        resizer.resize(&src_image, &mut dst_image, None)?;

        // save to file
        let mut buffer: Vec<u8> = Vec::new();
        let mut encoder = JpegEncoder::new(&mut buffer);
        encoder.encode(&dst_image.buffer(), size, size, src_image.color().into())?;
        let mut file = File::create(image_path(self.user_id, Some(size))).await?;
        file.write_all(&buffer).await?;

        Ok(())
    }
}

pub fn image_path(user_id: u32, size_opt: Option<u32>) -> PathBuf {
    let filename = match size_opt {
        Some(size) => format!("{}.{}.jpg", user_id, size),
        None => format!("{}.jpg", user_id),
    };
    PathBuf::from(IMAGE_PATH.to_string()).join(filename)
}
