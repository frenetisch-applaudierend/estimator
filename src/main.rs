use std::{error::Error, net::SocketAddr};

use axum::{response::Redirect, routing::get, Router};
use tower_http::{compression::CompressionLayer, services::ServeDir, trace::TraceLayer};

mod logging;
mod pages;

#[tokio::main]
async fn main() -> Result<(), impl Error> {
    logging::init();

    let app = Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/", get(Redirect::to("/projects")))
        .nest("/auth", pages::auth::routes())
        .nest("/projects", pages::projects::routes())
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
}
