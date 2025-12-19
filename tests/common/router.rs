use std::{clone, path::PathBuf, sync::Arc};

use axum::{
    Json, Router,
    body::Body,
    http::Request,
    response::{IntoResponse, Response},
};
use database::Database;
use reqwest::{Method, header};
use serde::Serialize;
use sqlx::SqlitePool;
use tower::ServiceExt;
use tower_sessions::{MemoryStore, Session, SessionManagerLayer, session::Id};
use zpi::{
    AppState, api_router, config::AppConfig, extractors::authenticated_user::AuthenticatedUser,
};

#[derive(Clone)]
pub struct AuthenticatedRouter {
    router: Router,
    cookie: String,
}

impl AuthenticatedRouter {
    pub async fn new(db: SqlitePool) -> Self {
        let _ = dotenvy::dotenv();
        let store = Arc::new(MemoryStore::default());

        let session_id = {
            let session = Session::new(Some(Id(1)), store.clone(), None);
            session
                .insert(
                    "user",
                    AuthenticatedUser {
                        id: 1,
                        username: "cheese".to_string(),
                        admin: true,
                    },
                )
                .await
                .unwrap();
            session.save().await.unwrap();
            session.id().unwrap()
        };

        let session_layer = SessionManagerLayer::new(Arc::into_inner(store).unwrap())
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
            cookie: format!("id={}", session_id),
        }
    }

    /// send a request to an endpoint on this router
    ///
    /// must have a leading "/"
    pub async fn get(self, path: &str) -> Response<Body> {
        self.request(Method::GET, path, None::<()>).await
    }

    /// send a patch request to an endpoint on this router
    ///
    /// must have a leading "/"
    pub async fn patch<T: Serialize>(self, path: &str, body: T) -> Response<Body> {
        self.request(Method::PATCH, path, Some(body)).await
    }

    /// send a patch request to an endpoint on this router
    ///
    /// must have a leading "/"
    pub async fn post<T: Serialize>(self, path: &str, body: T) -> Response<Body> {
        self.request(Method::POST, path, Some(body)).await
    }

    /// send a request to an endpoint on this router
    ///
    /// must have a leading "/"
    async fn request<T: Serialize>(
        self,
        method: Method,
        path: &str,
        body: Option<T>,
    ) -> Response<Body> {
        let request_builder = Request::builder()
            .method(method)
            .uri(path)
            .header(header::COOKIE, &self.cookie);

        let request = match body {
            Some(body) => request_builder
                .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Json(body).into_response().into_body()),
            None => request_builder.body(Body::empty()),
        };
        self.router.oneshot(request.unwrap()).await.unwrap()
    }
}
pub struct UnauthenticatedRouter {
    router: Router,
}

impl UnauthenticatedRouter {
    pub async fn new(db: SqlitePool) -> Self {
        let _ = dotenvy::dotenv();
        let store = MemoryStore::default();

        let session_layer = SessionManagerLayer::new(store)
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
        }
    }

    /// send a request to an endpoint on this router
    ///
    /// must have a leading "/"
    pub async fn get(self, path: &str) -> Response<Body> {
        self.router
            .oneshot(Request::builder().uri(path).body(Body::empty()).unwrap())
            .await
            .unwrap()
    }
}
