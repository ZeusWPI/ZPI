use axum::response::Html;
use reqwest::StatusCode;

static LOGIN_HTML: &str = include_str!("../templates/login.html");
static UPLOAD_HTML: &str = include_str!("../templates/upload.html");
static ERROR_HTML: &str = include_str!("../templates/error.html");

pub struct Page;
impl Page {
    pub fn upload(username: &str, user_id: u32) -> Html<String> {
        Html(
            UPLOAD_HTML
                .replace("{{version}}", env!("CARGO_PKG_VERSION"))
                .replace("{{username}}", username)
                .replace("{{user_id}}", &user_id.to_string()),
        )
    }

    pub fn login() -> Html<String> {
        Html(LOGIN_HTML.to_string())
    }

    pub fn error(status_code: StatusCode, msg: &str) -> Html<String> {
        Html(
            ERROR_HTML
                .replace("{{error_message}}", msg)
                .replace("{{status_code}}", &status_code.as_u16().to_string()),
        )
    }
}
