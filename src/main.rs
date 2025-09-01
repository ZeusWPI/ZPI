use std::error::Error;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};
use zpi::start_app;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = dotenvy::dotenv();

    // setup logging
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_env("LOG_LEVEL"))
        .init();

    start_app().await?;

    Ok(())
}
