use std::{env, path::PathBuf, sync::LazyLock};

use axum::{
    Json, Router,
    body::Body,
    extract::{Multipart, Path, Query},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    routing::get,
};
use rand::distr::{Alphanumeric, SampleString};
use serde::{Deserialize, Serialize};
use tokio::fs;
use tokio_util::io::ReaderStream;
use tower_http::trace::TraceLayer;
use tower_sessions::{MemoryStore, Session, SessionManagerLayer, cookie::SameSite};

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

static LOGIN_HTML: &str = include_str!("../static/login.html");
static LOGGEDIN_HTML: &str = include_str!("../static/loggedin.html");
static UPLOAD_HTML: &str = include_str!("../static/upload.html");

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let sess_store = MemoryStore::default();
    let sess_mw = SessionManagerLayer::new(sess_store)
        .with_same_site(SameSite::Lax)
        .with_http_only(false);

    let app = Router::new()
        .route("/", get(index))
        .route("/login", get(login))
        .route("/oauth/callback", get(callback))
        .route("/logout", get(logout))
        .route("/image", get(image_index).post(post_image))
        .route("/image/{id}", get(get_image))
        .layer(sess_mw)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub async fn index(session: Session) -> impl IntoResponse {
    match session.get::<ZauthUser>("user").await.unwrap() {
        None => Html(LOGIN_HTML.to_string()),
        Some(user) => {
            let html = LOGGEDIN_HTML.replace("{{username}}", &user.username);
            Html(html)
        }
    }
}

pub async fn image_index() -> impl IntoResponse {
    Html(UPLOAD_HTML)
}

pub async fn post_image(session: Session, mut multipart: Multipart) -> impl IntoResponse {
    match session.get::<ZauthUser>("user").await.unwrap() {
        None => Redirect::to("/login"),
        Some(user) => {
            while let Some(field) = multipart.next_field().await.unwrap() {
                if let Some("image_file") = field.name() {
                    let content_type = field.content_type().unwrap_or("").to_string();
                    if content_type != "image/jpeg" {
                        return Redirect::to("/");
                    }
                    let data = field.bytes().await.unwrap();

                    let path =
                        PathBuf::from(IMAGE_PATH.to_string()).join(user.id.to_string() + ".jpg");
                    fs::write(path, data).await.unwrap();
                    return Redirect::to(&("/image/".to_string() + &user.id.to_string()));
                }
            }
            Redirect::to("/image")
        }
    }
}

pub async fn get_image(Path(id): Path<u32>) -> impl IntoResponse {
    let path = std::path::PathBuf::from(IMAGE_PATH.to_string());
    let path = path.join(id.to_string() + ".jpg");
    dbg!(&path);
    let file = tokio::fs::File::open(path).await.unwrap();
    Body::from_stream(ReaderStream::new(file))
}

pub async fn login(session: Session) -> impl IntoResponse {
    let state = Alphanumeric.sample_string(&mut rand::rng(), 16);
    // insert state so we can check it in the callback
    session.insert("state", state.clone()).await.unwrap();
    // redirect to zauth to authenticate
    let zauth_url = ZAUTH_URL.to_string();
    let callback_url = CALLBACK_URL.to_string();
    let zauth_client_id = ZAUTH_CLIENT_ID.to_string();
    Redirect::to(&format!(
        "{zauth_url}/oauth/authorize?client_id={zauth_client_id}&response_type=code&state={state}&redirect_uri={callback_url}"
    ))
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
    id: i32,
    username: String,
}

pub async fn callback(Query(params): Query<Callback>, session: Session) -> impl IntoResponse {
    let zauth_state = session.get::<String>("state").await.unwrap().unwrap();

    // check if saved state matches returned state
    if zauth_state != params.state {
        return Redirect::to("/");
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
        .await
        .unwrap()
        .json::<ZauthToken>()
        .await
        .unwrap();

    // get user info from zauth
    let zauth_user = client
        .get(format!("{zauth_url}/current_user"))
        .header("Authorization", "Bearer ".to_owned() + &token.access_token)
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json::<ZauthUser>()
        .await
        .unwrap();

    session.clear().await;
    session.insert("user", zauth_user).await.unwrap();
    Redirect::to("/")
}
