use poem::{Endpoint, Route};
use poem_openapi::{OpenApi, OpenApiService};

mod auth;

pub fn routes(uri: &str) -> impl Endpoint {
    let uri = uri.replace("0.0.0.0", "localhost");
    let service =
        OpenApiService::new(auth::Api, "Estimator API", "1.0").server(format!("http://{uri}/api"));
    let ui = service.swagger_ui();

    Route::new().nest("/", service).nest("/swagger", ui)
}

struct Api;

#[OpenApi]
impl Api {}
