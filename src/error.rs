use axum::{
    extract::multipart::MultipartError,
    response::{Html, IntoResponse, Redirect, Response},
};
use image::ImageError;
use reqwest::Error as ReqwestError;
use reqwest::StatusCode;
use std::io::Error as IoError;
use thiserror::Error;
use tower_sessions::session::Error as TowerError;

use crate::pages::Page;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Session persistence error {0}")]
    Session(#[from] TowerError),

    #[error("Multipart form processing error {0}")]
    Multipart(#[from] MultipartError),

    #[error("I/O error")]
    Io(#[from] IoError),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("ZAuth authentication error: {0}")]
    Zauth(String),

    #[error("HTTP request error {0}")]
    Reqwest(#[from] ReqwestError),

    #[error("Image processing error")]
    Image(#[from] ImageError),

    #[error("ImageMagick command failed: {0}")]
    Magick(String),

    #[error("Submitted image resolution was too large")]
    ImageResTooLarge,

    #[error("The requested image was not found")]
    ImageNotFound,

    #[error("Submitted file had an incorrect type")]
    WrongFileType,

    #[error("No file was present in the multipart form")]
    NoFile,

    #[error("User was not logged in")]
    NotLoggedIn,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // log!
        tracing::error!("{}", self);

        match self {
            Self::NotLoggedIn => Redirect::to("/login").into_response(),
            _ => self.error_page().into_response(),
        }
    }
}

impl AppError {
    fn error_page(&self) -> (StatusCode, Html<String>) {
        let (status, msg) = match self {
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
            Self::ImageNotFound => (StatusCode::NOT_FOUND, "We couldn't find that image."),

            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Please help I have internal errors. D:",
            ),
        };

        (status, Page::error(status, msg))
    }
}
