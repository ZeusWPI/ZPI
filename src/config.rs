use std::{env, path::PathBuf};

use crate::error::AppError;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub zauth_url: String,
    pub zauth_callback: String,
    pub zauth_client_id: String,
    pub zauth_client_secret: String,

    pub image_path: PathBuf,
    pub magick_path: String,

    pub database_url: String,

    pub frontend_url: String,
}

impl AppConfig {
    pub fn load() -> Result<Self, AppError> {
        Ok(Self {
            zauth_url: get_env_var("ZAUTH_URL")?,
            zauth_callback: get_env_var("ZAUTH_CALLBACK_PATH")?,
            zauth_client_id: get_env_var("ZAUTH_CLIENT_ID")?,
            zauth_client_secret: get_env_var("ZAUTH_CLIENT_SECRET")?,
            image_path: get_env_var_path("IMAGE_PATH")?,
            magick_path: get_env_var("MAGICK_PATH")?,
            database_url: get_env_var("DATABASE_URL")?,
            frontend_url: get_env_var("FRONTEND_URL")?,
        })
    }
}

fn get_env_var(name: &str) -> Result<String, AppError> {
    env::var(name).map_err(|_| AppError::Env(name.to_string()))
}

fn get_env_var_path(name: &str) -> Result<PathBuf, AppError> {
    Ok(PathBuf::from(
        env::var(name).map_err(|_| AppError::Env(name.to_string()))?,
    ))
}
