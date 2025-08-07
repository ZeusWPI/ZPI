use std::{env, path::PathBuf, sync::LazyLock};

use axum::{
    Router,
    body::Body,
    extract::{DefaultBodyLimit, Multipart, Path, Query},
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
};
use error::AppError;
use pages::Page;
use rand::distr::{Alphanumeric, SampleString};
use serde::{Deserialize, Serialize};
use tokio::fs;
use tokio_util::io::ReaderStream;
use tower_http::trace::TraceLayer;
use tower_sessions::{MemoryStore, Session, SessionManagerLayer, cookie::SameSite};

mod error;
mod pages;

static ZAUTH_URL: LazyLock<String> =
    LazyLock::new(|| env::var("ZAUTH_URL").expect("ZAUTH_URL not present"));
static CALLBACK_URL: LazyLock<String> =
    LazyLock::new(|| env::var("ZAUTH_CALLBACK_PATH").expect("ZAUTH_CALLBACK_PATH not present"));
static ZAUTH_CLIENT_ID: LazyLock<String> =
    LazyLock::new(|| env::var("ZAUTH_CLIENT_ID").expect("ZAUTH_CLIENT_ID not present"));
static ZAUTH_CLIENT_SECRET: LazyLock<String> =
    LazyLock::new(|| env::var("ZAUTH_CLIENT_SECRET").expect("ZAUTH_CLIENT_SECRET not present"));
static IMAGE_PATH: LazyLock<String> =
    LazyLock::new(|| env::var("IMAGE_PATH").expect("IMAGE_PATH not present"));

static PLACEHOLDER: &[u8] = include_bytes!("../static/placeholder.jpg");

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let sess_store = MemoryStore::default();
    let sess_mw = SessionManagerLayer::new(sess_store).with_same_site(SameSite::Lax);

    let app = Router::new()
        .route("/", get(index))
        .route("/login", get(login))
        .route("/oauth/callback", get(callback))
        .route("/logout", get(logout))
        .route("/image", post(post_image))
        .route("/image/{id}", get(get_image))
        .route("/{*wildcard}", get(|| async { Page::error("404") }))
        .layer(sess_mw)
        .layer(DefaultBodyLimit::max(10_485_760))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub async fn index(session: Session) -> Result<Html<String>, AppError> {
    Ok(match session.get::<ZauthUser>("user").await? {
        None => Page::login(),
        Some(user) => Page::upload(&user.username, user.id),
    })
}

pub async fn post_image(session: Session, mut multipart: Multipart) -> Result<Redirect, AppError> {
    match session.get::<ZauthUser>("user").await? {
        None => Ok(Redirect::to("/")),
        Some(user) => {
            while let Some(field) = multipart.next_field().await? {
                if let Some("image_file") = field.name() {
                    let data = field.bytes().await?;
                    match infer::get(&data)
                        .ok_or(AppError::WrongFileType)?
                        .mime_type()
                    {
                        "image/jpeg" => {}
                        _ => return Err(AppError::WrongFileType),
                    }

                    let path = image_path(user.id);
                    fs::write(path, data).await?;
                    return Ok(Redirect::to("/"));
                }
            }
            Err(AppError::NoFile)
        }
    }
}

#[derive(Deserialize)]
pub struct PlaceholderQuery {
    placeholder: Option<bool>,
}

pub async fn get_image(
    Query(params): Query<PlaceholderQuery>,
    Path(id): Path<u32>,
) -> Result<Body, AppError> {
    let path = image_path(id);
    match tokio::fs::File::open(path).await {
        Err(_) => match params.placeholder {
            Some(true) => Ok(Body::from(PLACEHOLDER)),
            _ => Err(AppError::ImageNotFound),
        },
        Ok(file) => Ok(Body::from_stream(ReaderStream::new(file))),
    }
}

pub async fn login(session: Session) -> Result<Redirect, AppError> {
    let state = Alphanumeric.sample_string(&mut rand::rng(), 16);
    // insert state so we can check it in the callback
    session.insert("state", state.clone()).await?;
    // redirect to zauth to authenticate
    let zauth_url = ZAUTH_URL.to_string();
    let callback_url = CALLBACK_URL.to_string();
    let zauth_client_id = ZAUTH_CLIENT_ID.to_string();
    Ok(Redirect::to(&format!(
        "{zauth_url}/oauth/authorize?client_id={zauth_client_id}&response_type=code&state={state}&redirect_uri={callback_url}"
    )))
}

pub async fn logout(session: Session) -> impl IntoResponse {
    session.clear().await;
    Redirect::to("/")
}

#[derive(Deserialize, Debug)]
pub struct Callback {
    state: String,
    code: String,
}

#[derive(Deserialize, Debug)]
pub struct ZauthToken {
    access_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ZauthUser {
    id: u32,
    username: String,
}

pub async fn callback(
    Query(params): Query<Callback>,
    session: Session,
) -> Result<Redirect, AppError> {
    let zauth_state = match session.get::<String>("state").await? {
        None => return Ok(Redirect::to("/login")),
        Some(v) => v,
    };

    // check if saved state matches returned state
    if zauth_state != params.state {
        return Err(AppError::Zauth("States don't match".into()));
    }

    let callback_url = CALLBACK_URL.to_string();
    let client = reqwest::Client::new();
    let form = [
        ("grant_type", "authorization_code"),
        ("code", &params.code),
        ("redirect_uri", &callback_url),
    ];

    let zauth_url = ZAUTH_URL.to_string();
    // get token from zauth with code
    let token = client
        .post(format!("{zauth_url}/oauth/token"))
        .basic_auth(
            ZAUTH_CLIENT_ID.to_string(),
            Some(ZAUTH_CLIENT_SECRET.to_string()),
        )
        .form(&form)
        .send()
        .await?
        .json::<ZauthToken>()
        .await?;

    // get user info from zauth
    let zauth_user = client
        .get(format!("{zauth_url}/current_user"))
        .header("Authorization", "Bearer ".to_owned() + &token.access_token)
        .send()
        .await?
        .error_for_status()?
        .json::<ZauthUser>()
        .await?;

    session.clear().await;
    session.insert("user", zauth_user).await?;
    Ok(Redirect::to("/"))
}

fn image_path(user_id: u32) -> PathBuf {
    PathBuf::from(IMAGE_PATH.to_string()).join(user_id.to_string() + ".jpg")
}
