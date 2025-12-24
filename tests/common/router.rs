use std::{path::PathBuf, sync::Arc};

use axum::{
    Json,
    body::Body,
    http::Request,
    response::{IntoResponse, Response},
};
use database::Database;
use dotenvy::dotenv;
use reqwest::{Method, header};
use serde::Serialize;
use sqlx::SqlitePool;
use tower::ServiceExt;
use tower_sessions::{MemoryStore, Session, SessionManagerLayer, session::Id};
use zpi::{
    AppState, api_router, config::AppConfig, extractors::authenticated_user::AuthenticatedUser,
};

pub struct TestRouter {
    router: axum::Router,
    store: MemoryStore,
    cookie: Option<String>,
    api_key: Option<String>,
}

impl TestRouter {
    pub fn new(db: SqlitePool) -> Self {
        let _ = dotenv();
        let store = MemoryStore::default();

        let session_layer = SessionManagerLayer::new(store.clone())
            .with_secure(false)
            .with_same_site(tower_sessions::cookie::SameSite::Lax);

        let mut config = AppConfig::load().unwrap();
        config.image_path = PathBuf::from("./tests/test_images");

        let state = AppState {
            db: Database::new(db),
            config,
        };

        Self {
            router: api_router().layer(session_layer).with_state(state),
            store: store,
            cookie: None,
            api_key: None,
        }
    }

    pub async fn as_user(db: SqlitePool) -> Self {
        Self::new(db)
            .add_to_store(AuthenticatedUser {
                id: 1,
                username: "cheese".to_string(),
                admin: false,
            })
            .await
    }

    pub async fn as_admin(db: SqlitePool) -> Self {
        Self::new(db)
            .add_to_store(AuthenticatedUser {
                id: 1,
                username: "cheese".to_string(),
                admin: true,
            })
            .await
    }

    async fn add_to_store(mut self, user: AuthenticatedUser) -> Self {
        let session = Session::new(Some(Id(1)), Arc::new(self.store.clone()), None);
        session.insert("user", user).await.unwrap();
        session.save().await.unwrap();
        self.cookie.replace(format!("id={}", session.id().unwrap()));
        self
    }

    pub async fn with_api_key(db: SqlitePool, api_key: &str) -> Self {
        let mut router = Self::new(db);
        router.api_key = Some("Bearer ".to_string() + api_key);
        router
    }

    /// send a request to an endpoint on this router
    ///
    /// must have a leading "/"
    pub async fn get(&self, path: &str) -> Response<Body> {
        self.request(Method::GET, path, None::<()>).await
    }

    /// send a patch request to an endpoint on this router
    ///
    /// must have a leading "/"
    pub async fn patch<T: Serialize>(&self, path: &str, body: T) -> Response<Body> {
        self.request(Method::PATCH, path, Some(body)).await
    }

    /// send a post request to an endpoint on this router
    ///
    /// must have a leading "/"
    pub async fn post<T: Serialize>(&self, path: &str, body: T) -> Response<Body> {
        self.request(Method::POST, path, Some(body)).await
    }

    /// send a request to an endpoint on this router
    ///
    /// must have a leading "/"
    async fn request<T: Serialize>(
        &self,
        method: Method,
        path: &str,
        body: Option<T>,
    ) -> Response<Body> {
        let mut request_builder = Request::builder().method(method).uri(path);

        if let Some(api_key) = &self.api_key {
            request_builder = request_builder.header(header::AUTHORIZATION, api_key);
        }

        if let Some(cookie) = &self.cookie {
            request_builder = request_builder.header(header::COOKIE, cookie);
        }

        let request = match body {
            Some(body) => request_builder
                .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Json(body).into_response().into_body()),
            None => request_builder.body(Body::empty()),
        };

        self.router.clone().oneshot(request.unwrap()).await.unwrap()
    }
}
