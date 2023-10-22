use poem_openapi::{payload::PlainText, OpenApi};

pub struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/auth/login", method = "post")]
    async fn login(&self) -> PlainText<&'static str> {
        PlainText("auth/login")
    }
}
