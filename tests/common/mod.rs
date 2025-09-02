use std::sync::Arc;

use axum::{
    body::Body, debug_handler, http::{Request, Response}, response::IntoResponse, Json, Router
};
use reqwest::header;
use sqlx::SqlitePool;
use tower::ServiceExt;
use tower_sessions::{MemoryStore, Session, SessionManagerLayer, session::Id};
use zpi::{
    create_router,
    models::user::{User, UserPatchPayload},
};

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

    // TODO methode generiek maken
    /// send a patch request to an endpoint on this router
    ///
    /// must have a leading "/"
    pub async fn patch(self, path: &str, body: Json<UserPatchPayload>) -> Response<Body> {
        self.router
            .oneshot(dbg!(
                Request::builder()
                    .method("PATCH")
                    .uri(path)
                    .header(header::COOKIE, &self.cookie)
                    .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body.into_response().into_body())
                    .unwrap(),
            ))
            .await
            .unwrap()
    }
}
