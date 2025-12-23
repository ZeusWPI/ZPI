use axum::{
    extract::multipart::MultipartError,
    response::{IntoResponse, Response},
};
use database::error::DatabaseError;
use reqwest::Error as ReqwestError;
use reqwest::StatusCode;
use std::io::Error as IoError;
use thiserror::Error;
use tokio::task::JoinError;
use tower_sessions::session::Error as TowerError;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Session persistence error {0}")]
    Session(#[from] TowerError),

    #[error("Multipart form processing error {0}")]
    Multipart(#[from] MultipartError),

    #[error("I/O error")]
    Io(#[from] IoError),

    #[error("Thread join error: {0}")]
    Join(#[from] JoinError),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("ZAuth authentication error: {0}")]
    Zauth(String),

    #[error("HTTP request error {0}")]
    Reqwest(#[from] ReqwestError),

    #[error("Database error: {0}")]
    Database(DatabaseError),

    #[error("ImageMagick command failed: {0}")]
    Magick(String),

    #[error("Env var {0} not set :(")]
    Env(String),

    #[error("Axum error: {0}")]
    Axum(#[from] axum::Error),

    #[error("Submitted image resolution was too large")]
    ImageResTooLarge,

    #[error("The requested image was not found")]
    NotFound,

    #[error("Submitted file had an incorrect type")]
    WrongFileType,

    #[error("No file was present in the multipart form")]
    NoFile,

    #[error("User was not logged in")]
    NotLoggedIn,

    #[error("Wrong api key")]
    BadApiKey,

    #[error("Forbidden")]
    Forbidden,

    #[error("Payload error: {0}")]
    PayloadError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // log!
        tracing::error!("{}", self);
        self.error_page().into_response()
    }
}

impl AppError {
    fn error_page(&self) -> (StatusCode, &'static str) {
        let (status, msg) = match self {
            Self::PayloadError(_) => (StatusCode::BAD_REQUEST, "Payload error"),
            Self::NotLoggedIn => (StatusCode::UNAUTHORIZED, "Not logged in."),
            Self::BadApiKey => (StatusCode::UNAUTHORIZED, "Bad api key."),
            Self::Forbidden => (StatusCode::FORBIDDEN, "Forbidden."),
            Self::NoFile => (
                StatusCode::BAD_REQUEST,
                "No file found in request. Please select an image.",
            ),
            Self::Multipart(_) => (
                StatusCode::BAD_REQUEST,
                "There was a problem with your file upload. Please try again.",
            ),
            Self::ImageResTooLarge => (
                StatusCode::PAYLOAD_TOO_LARGE,
                "The image resolution is too large. Maximum is 10k x 10k pixels.",
            ),
            Self::WrongFileType => (
                StatusCode::BAD_REQUEST,
                "Incorrect file type. Please upload a JPG, PNG, GIF, or WEBP file.",
            ),
            Self::NotFound => (StatusCode::NOT_FOUND, "We couldn't find that."),

            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Please help I have internal errors. D:",
            ),
        };

        (status, msg)
    }
}

impl From<DatabaseError> for AppError {
    fn from(value: DatabaseError) -> Self {
        match value {
            DatabaseError::NotFound => Self::NotFound,
            other => Self::Database(other),
        }
    }
}
