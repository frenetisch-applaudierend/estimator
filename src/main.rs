use std::{error::Error, net::SocketAddr};

use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use sailfish::TemplateOnce;
use tower_http::{services::ServeDir, trace::TraceLayer};

mod logging;

#[tokio::main]
async fn main() -> Result<(), impl Error> {
    logging::init();

    let app = Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/", get(hello))
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
}

#[derive(sailfish::TemplateOnce)]
#[template(path = "layout.stpl")]
struct Layout<T: TemplateOnce> {
    lang: Option<String>,
    title: String,
    content: T,
}

#[derive(sailfish::TemplateOnce)]
#[template(path = "hello.stpl")]
struct Hello {
    messages: Vec<String>,
}

async fn hello() -> impl IntoResponse {
    let html = Layout {
        lang: Some("de".to_string()),
        title: "Hello, World!".to_string(),
        content: Hello {
            messages: vec!["Hello".to_string(), "World".to_string()],
        },
    };

    Html(html.render_once().expect("Could not render template"))
}
