use axum::{routing::get, Router};

pub fn routes<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new().route("/", get(list))
}

async fn list() -> &'static str {
    "Project list"
}
