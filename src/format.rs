use image::ImageFormat;

use crate::error::AppError;

#[derive(Clone, Copy)]
pub enum SupportedFormat {
    Jpeg,
    Png,
    Webp,
}

impl SupportedFormat {
    pub fn guess(data: &[u8]) -> Result<SupportedFormat, AppError> {
        match image::guess_format(data)? {
            ImageFormat::Jpeg => Ok(Self::Jpeg),
            ImageFormat::Png => Ok(Self::Png),
            ImageFormat::WebP => Ok(Self::Webp),
            _ => Err(AppError::WrongFileType),
        }
    }

    pub fn extension(self) -> &'static str {
        match self {
            Self::Jpeg => "jpg",
            Self::Png => "png",
            Self::Webp => "webp",
        }
    }

    pub fn mime_type(self) -> &'static str {
        match self {
            Self::Jpeg => "image/jpeg",
            Self::Png => "image/png",
            Self::Webp => "image/webp",
        }
    }
}

impl From<SupportedFormat> for ImageFormat {
    fn from(val: SupportedFormat) -> Self {
        match val {
            SupportedFormat::Jpeg => ImageFormat::Jpeg,
            SupportedFormat::Png => ImageFormat::Png,
            SupportedFormat::Webp => ImageFormat::WebP,
        }
    }
}
