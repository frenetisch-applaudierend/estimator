use poem::{listener::TcpListener, Route, Server};

mod cli;
mod logging;
mod spa;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let options = cli::init();
    logging::init();

    let spa = spa::SpaEndpoint::new(options.assets_dir)?;
    let app = Route::new().nest("/", spa);

    let endpoint = format!("{}:{}", options.host, options.port);
    tracing::info!("You can view the website at: http://{}/", endpoint);

    Server::new(TcpListener::bind(endpoint)).run(app).await
}
