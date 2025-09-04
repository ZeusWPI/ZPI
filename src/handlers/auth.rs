use axum::{
    extract::Query,
    response::{IntoResponse, Redirect},
};
use database::{Database, models::user::UserCreatePayload};
use rand::distr::{Alphanumeric, SampleString};
use serde::{Deserialize, Serialize};
use tower_sessions::Session;

use crate::{config::AppConfig, error::AppError, handlers::AuthenticatedUser};

pub struct AuthHandler;

impl AuthHandler {
    pub async fn login(session: Session, config: AppConfig) -> Result<Redirect, AppError> {
        let zauth_state = Alphanumeric.sample_string(&mut rand::rng(), 16);
        // insert state so we can check it in the callback
        session.insert("state", zauth_state.clone()).await?;
        // redirect to zauth to authenticate
        let zauth_url = config.zauth_url;
        let callback_url = config.zauth_callback;
        let zauth_client_id = config.zauth_client_id;
        Ok(Redirect::to(&format!(
            "{zauth_url}/oauth/authorize?client_id={zauth_client_id}&response_type=code&state={zauth_state}&redirect_uri={callback_url}"
        )))
    }

    pub async fn logout(session: Session, config: AppConfig) -> impl IntoResponse {
        session.clear().await;
        Redirect::to(&config.frontend_url)
    }

    pub async fn callback(
        Query(params): Query<Callback>,
        session: Session,
        config: AppConfig,
        db: Database,
    ) -> Result<Redirect, AppError> {
        let zauth_state = match session.get::<String>("state").await? {
            None => return Ok(Redirect::to("/login")),
            Some(v) => v,
        };

        // check if saved state matches returned state
        if zauth_state != params.state {
            return Err(AppError::Zauth("States don't match".into()));
        }

        let client = reqwest::Client::new();
        let form = [
            ("grant_type", "authorization_code"),
            ("code", &params.code),
            ("redirect_uri", &config.zauth_callback),
        ];

        // get token from zauth with code
        let token = client
            .post(format!("{}/oauth/token", config.zauth_url.as_str()))
            .basic_auth(config.zauth_client_id, Some(config.zauth_client_secret))
            .form(&form)
            .send()
            .await?
            .json::<ZauthToken>()
            .await?;

        // get user info from zauth
        let zauth_user = client
            .get(format!("{}/current_user", config.zauth_url))
            .header("Authorization", "Bearer ".to_owned() + &token.access_token)
            .send()
            .await?
            .error_for_status()?
            .json::<ZauthUser>()
            .await?;

        let user = db.users().create(zauth_user.into()).await?;

        session.clear().await;
        session
            .insert("user", AuthenticatedUser::from(user))
            .await?;
        Ok(Redirect::to(&config.frontend_url))
    }
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
    pub id: u32,
    pub username: String,
}

impl From<ZauthUser> for UserCreatePayload {
    fn from(value: ZauthUser) -> Self {
        Self {
            id: value.id,
            username: value.username,
        }
    }
}
