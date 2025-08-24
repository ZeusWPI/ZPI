use axum::{
    extract::multipart::MultipartError,
    response::{IntoResponse, Redirect, Response},
};
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
    Magick(String),
    ImageResTooLarge,
    ImageNotFound,
    WrongFileType,
    NoFile,
    NotLoggedIn,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::NotLoggedIn => Redirect::to("/login").into_response(),
            AppError::Session(err) => {
                tracing::error!("Session error {}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Page::error("Session error"),
                )
                    .into_response()
            }
            AppError::Zauth(err) => {
                tracing::error!("Zauth error {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Zauth error").into_response()
            }
            AppError::Reqwest(err) => {
                tracing::error!("Reqwest error {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Reqwest error").into_response()
            }
            AppError::Io(err) => {
                tracing::error!("IO error {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Io error").into_response()
            }
            AppError::Internal(err) => {
                tracing::error!("Internal error {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal error").into_response()
            }
            AppError::Multipart(err) => {
                tracing::error!("Multipart error {}", err);
                (StatusCode::BAD_REQUEST, "Multipart error").into_response()
            }
            AppError::ImageResTooLarge => (
                StatusCode::BAD_REQUEST,
                "Maximum image resolution is 10k x 10k pixels",
            )
                .into_response(),
            AppError::Image(err) => {
                tracing::error!("Image error {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Image error").into_response()
            }
            AppError::Magick(stderr) => {
                tracing::error!("Magick error {}", stderr);
                (StatusCode::INTERNAL_SERVER_ERROR, "Error with magick").into_response()
            }
            AppError::WrongFileType => {
                (StatusCode::BAD_REQUEST, "Please give a jpg file").into_response()
            }
            AppError::ImageNotFound => {
                (StatusCode::NOT_FOUND, "No image for this user").into_response()
            }
            AppError::NoFile => (StatusCode::BAD_REQUEST, "Please give a file").into_response(),
        }
    }
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
