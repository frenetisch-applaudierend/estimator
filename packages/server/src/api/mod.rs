use poem::{get, middleware::CookieJarManager, Endpoint, EndpointExt, Route};
use poem_openapi::OpenApi;

mod auth;

pub fn routes() -> impl Endpoint {
    Route::new()
        .at("/auth/login", get(auth::login))
        .with(CookieJarManager::new())
}

struct Api;

#[OpenApi]
impl Api {}
