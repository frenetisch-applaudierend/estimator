use axum::{http::StatusCode, response::Html};
use sailfish::TemplateOnce;

pub mod auth;
pub mod projects;

#[derive(TemplateOnce)]
#[template(path = "layout.html")]
struct Layout<T: TemplateOnce> {
    lang: Option<String>,
    title: String,
    content: T,
}

#[derive(TemplateOnce)]
#[template(path = "app.html")]
struct App<T: TemplateOnce> {
    content: T,
}

impl<T: TemplateOnce> axum::response::IntoResponse for Layout<T> {
    fn into_response(self) -> axum::response::Response {
        match self.render_once() {
            Ok(content) => Html(content).into_response(),
            Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
        }
    }
}
