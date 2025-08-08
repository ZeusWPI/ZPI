use std::{env, path::PathBuf, sync::LazyLock};

pub use image::ImageFormat;

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
    format: SupportedFormat,
}

impl<'a> ZPIImage<'a> {
    /// Creates a ZPIImage from data. Checks if it is a supported format.
    pub fn from_data(data: &'a [u8], user_id: u32) -> Result<Self, AppError> {
        let format = SupportedFormat::guess(data)?;
        Ok(ZPIImage {
            data,
            user_id,
            format,
        })
    }

    pub async fn save_original(&self) -> Result<(), AppError> {
        fs::write(self.path(None, self.format), self.data).await?;
        Ok(())
    }

    pub async fn save_multiple_resized(&self, sizes: &[u32]) -> Result<(), AppError> {
        for size in sizes {
            self.save_resized(*size).await?;
        }

        Ok(())
    }

    pub async fn save_resized(&self, size: u32) -> Result<(), AppError> {
        // load image from memory data
        let src_image = image::load_from_memory_with_format(self.data, self.format.into())?;

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

        // write resized image to buffer
        let mut buffer: Vec<u8> = Vec::new();
        let mut encoder = JpegEncoder::new(&mut buffer);
        encoder.encode(dst_image.buffer(), size, size, src_image.color().into())?;

        // save image buffer to file
        let mut file = File::create(self.path(Some(size), SupportedFormat::Jpeg)).await?;
        file.write_all(&buffer).await?;

        Ok(())
    }

    pub fn path(&self, size_opt: Option<u32>, format: SupportedFormat) -> PathBuf {
        let filename = match size_opt {
            Some(size) => format!("{}.{}.{}", self.user_id, size, format.extension()),
            None => format!("{}.{}", self.user_id, format.extension()),
        };
        PathBuf::from(IMAGE_PATH.to_string()).join(filename)
    }
}

pub fn jpg_image_path(user_id: u32, size_opt: Option<u32>) -> PathBuf {
    let filename = match size_opt {
        Some(size) => format!("{user_id}.{size}.jpg"),
        None => format!("{user_id}.jpg"),
    };
    PathBuf::from(IMAGE_PATH.to_string()).join(filename)
}

#[derive(Clone, Copy)]
pub enum SupportedFormat {
    Jpeg,
    Png,
}

impl SupportedFormat {
    pub fn guess(data: &[u8]) -> Result<SupportedFormat, AppError> {
        match image::guess_format(data).map_err(AppError::Image)? {
            ImageFormat::Jpeg => Ok(SupportedFormat::Jpeg),
            ImageFormat::Png => Ok(SupportedFormat::Png),
            _ => Err(AppError::WrongFileType),
        }
    }

    pub fn extension(self) -> &'static str {
        match self {
            SupportedFormat::Jpeg => "jpg",
            SupportedFormat::Png => "png",
        }
    }

    pub fn mime_type(self) -> &'static str {
        match self {
            SupportedFormat::Jpeg => "image/jpeg",
            SupportedFormat::Png => "image/png",
        }
    }
}

impl From<SupportedFormat> for ImageFormat {
    fn from(val: SupportedFormat) -> Self {
        match val {
            SupportedFormat::Jpeg => ImageFormat::Jpeg,
            SupportedFormat::Png => ImageFormat::Png,
        }
    }
}
