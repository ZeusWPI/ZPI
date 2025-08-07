use axum::response::Html;

static LOGIN_HTML: &str = include_str!("../static/login.html");
static UPLOAD_HTML: &str = include_str!("../static/upload.html");
static ERROR_HTML: &str = include_str!("../static/error.html");

pub struct Page;
impl Page {
    pub fn upload(username: &str, user_id: u32) -> Html<String> {
        Html(
            UPLOAD_HTML
                .replace("{{username}}", username)
                .replace("{{user_id}}", &user_id.to_string()),
        )
    }

    pub fn login() -> Html<String> {
        Html(LOGIN_HTML.to_string())
    }

    pub fn error(msg: &str) -> Html<String> {
        Html(ERROR_HTML.replace("{{error_message}}", msg))
    }
}
