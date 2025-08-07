use axum::{
    extract::multipart::MultipartError,
    response::{IntoResponse, Response},
};
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

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (code, message) = match self {
            AppError::Session(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Session error"),
            AppError::Zauth(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Zauth error"),
            AppError::Reqwest(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Reqwest error"),
            AppError::Io(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Io error"),
            AppError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal error"),
            AppError::Multipart(_) => (StatusCode::BAD_REQUEST, "Multipart error"),
            AppError::WrongFileType => (StatusCode::BAD_REQUEST, "Please give a jpg file"),
            AppError::ImageNotFound => (StatusCode::NOT_FOUND, "No image for this user"),
            AppError::NoFile => (StatusCode::BAD_REQUEST, "Please give a file"),
        };
        (code, Page::error(message)).into_response()
    }
}
