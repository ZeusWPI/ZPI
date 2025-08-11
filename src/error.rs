use axum::{
    extract::multipart::MultipartError,
    response::{IntoResponse, Response},
};
use fast_image_resize::ResizeError;
use image::ImageError;
use reqwest::Error as ReqwestError;
use reqwest::StatusCode;
use std::io::Error as IoError;
use tower_sessions::session::Error as TowerError;

use crate::pages::Page;

pub enum AppError {
    Session(TowerError),
    Multipart(MultipartError),
    Io(IoError),
    Internal(String),
    Zauth(String),
    Reqwest(ReqwestError),
    Image(ImageError),
    Resize(ResizeError),
    ImageResTooLarge,
    ImageNotFound,
    WrongFileType,
    NoFile,
}

impl From<TowerError> for AppError {
    fn from(value: TowerError) -> Self {
        AppError::Session(value)
    }
}

impl From<MultipartError> for AppError {
    fn from(value: MultipartError) -> Self {
        AppError::Multipart(value)
    }
}

impl From<IoError> for AppError {
    fn from(value: IoError) -> Self {
        AppError::Io(value)
    }
}

impl From<ReqwestError> for AppError {
    fn from(value: ReqwestError) -> Self {
        AppError::Reqwest(value)
    }
}

impl From<ImageError> for AppError {
    fn from(value: ImageError) -> Self {
        AppError::Image(value)
    }
}

impl From<ResizeError> for AppError {
    fn from(value: ResizeError) -> Self {
        AppError::Resize(value)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (code, message) = match self {
            AppError::Session(err) => {
                tracing::error!("Session error {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Session error")
            }
            AppError::Zauth(err) => {
                tracing::error!("Zauth error {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Zauth error")
            }
            AppError::Reqwest(err) => {
                tracing::error!("Reqwest error {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Reqwest error")
            }
            AppError::Io(err) => {
                tracing::error!("Io error {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Io error")
            }
            AppError::Internal(err) => {
                tracing::error!("Internal error {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal error")
            }
            AppError::Multipart(err) => {
                tracing::error!("Multipart error {}", err);
                (StatusCode::BAD_REQUEST, "Multipart error")
            }
            AppError::ImageResTooLarge => (
                StatusCode::BAD_REQUEST,
                "Maximum image resolution is 10k x 10k pixels",
            ),
            AppError::Resize(err) => {
                tracing::error!("Resize error {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Resize error")
            }
            AppError::Image(err) => {
                tracing::error!("Image error {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Image error")
            }
            AppError::WrongFileType => (StatusCode::BAD_REQUEST, "Please give a jpg file"),
            AppError::ImageNotFound => (StatusCode::NOT_FOUND, "No image for this user"),
            AppError::NoFile => (StatusCode::BAD_REQUEST, "Please give a file"),
        };
        (code, Page::error(message)).into_response()
    }
}
