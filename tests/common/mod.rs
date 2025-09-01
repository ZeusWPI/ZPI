use std::sync::Arc;

use axum::{
    Router,
    body::Body,
    http::{Request, Response},
};
use reqwest::header;
use sqlx::SqlitePool;
use tower::ServiceExt;
use tower_sessions::{MemoryStore, Session, SessionManagerLayer, session::Id};
use zpi::{create_router, models::user::User};

pub struct AuthenticatedRouter {
    router: Router,
    cookie: String,
}

impl AuthenticatedRouter {
    pub async fn new(db: SqlitePool) -> Self {
        let store = Arc::new(MemoryStore::default());

        let session_id = {
            let session = Session::new(Some(Id(1)), store.clone(), None);
            session
                .insert(
                    "user",
                    User {
                        id: 1,
                        username: "cheese".to_string(),
                        about: "Just a test user, doing its job... and fantasizing about a life outside the test environment.".to_string(),
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

        Self {
            router: create_router().layer(session_layer).with_state(db),
            cookie: format!("id={}", session_id),
        }
    }

    /// send a request to an endpoint on this router
    ///
    /// must have a leading "/"
    pub async fn get(self, path: &str) -> Response<Body> {
        self.router
            .oneshot(
                Request::builder()
                    .uri(path)
                    .header(header::COOKIE, &self.cookie)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap()
    }
}
