use axum::{response::IntoResponse, routing::get, Router};
use sailfish::TemplateOnce;

use super::{App, Layout};

pub fn routes<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new().route("/", get(list))
}

#[derive(TemplateOnce)]
#[template(path = "projects/list.html")]
struct ProjectsList;

async fn list() -> impl IntoResponse {
    Layout {
        title: "Projects".to_string(),
        lang: None,
        content: App {
            content: ProjectsList,
        },
    }
}
