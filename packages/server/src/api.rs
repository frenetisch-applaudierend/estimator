use poem::{get, handler, web::Path, Route};

pub fn routes() -> Route {
    Route::new().at("/hello/:name", get(hello))
}

#[handler]
fn hello(Path(name): Path<String>) -> String {
    format!("hello: {}", name)
}
