use axum::Json;
use serde::Serialize;

pub struct VersionHandler;

#[derive(Serialize)]
pub struct VersionResponse {
    version: &'static str,
}

impl VersionHandler {
    pub async fn get() -> Json<VersionResponse> {
        Json(VersionResponse {
            version: env!("CARGO_PKG_VERSION"),
        })
    }
}
