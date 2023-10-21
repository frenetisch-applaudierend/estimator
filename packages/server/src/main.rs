use poem::Route;

mod cli;
mod logging;
mod spa;

#[tokio::main]
async fn main() {
    let options = cli::init();
    logging::init();

    let app = Router::new().fallback_service(spa::serve(options.assets_dir).await);

    let endpoint = format!("{}:{}", options.host, options.port);
    tracing::info!("You can view the website at: http://{}/", endpoint);

    axum::Server::bind(&endpoint.parse().expect("Could not bind server"))
        .serve(app.into_make_service())
        .await
        .expect("Could not start server");
}
