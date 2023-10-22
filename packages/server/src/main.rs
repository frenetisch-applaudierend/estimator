use poem::{listener::TcpListener, middleware::CookieJarManager, EndpointExt, Route, Server};

mod api;
mod cli;
mod logging;
mod spa;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let options = cli::init();
    logging::init();

    let endpoint = format!("{}:{}", options.host, options.port);

    let spa = spa::SpaEndpoint::setup(options.assets_dir)?;
    let app = Route::new()
        .nest("/", spa)
        .nest("/api", api::routes(&endpoint))
        .with(CookieJarManager::new());

    Server::new(TcpListener::bind(endpoint)).run(app).await
}
