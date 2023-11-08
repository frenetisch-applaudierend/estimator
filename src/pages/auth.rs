use axum::{
    response::{IntoResponse, Redirect},
    routing::get,
    Form, Router,
};
use sailfish::TemplateOnce;
use serde::Deserialize;

use super::Layout;

pub fn routes<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new().route("/login", get(login_page).post(handle_login))
}

#[derive(TemplateOnce)]
#[template(path = "auth/login.html")]
struct LoginPage {
    email: Option<String>,
    error: Option<String>,
}

async fn login_page() -> impl IntoResponse {
    Layout {
        title: "Login".to_string(),
        lang: None,
        content: LoginPage {
            email: None,
            error: None,
        },
    }
}

#[derive(Clone, Debug, Deserialize)]
struct LoginFormData {
    email: String,
    password: String,
}

async fn handle_login(
    Form(login_form): Form<LoginFormData>,
) -> Result<Redirect, Layout<LoginPage>> {
    tracing::info!("Logging in as {:?}", login_form);

    if login_form.email == "markus.gasser@mailbox.org" && login_form.password == "vista7" {
        Ok(Redirect::to("/projects"))
    } else {
        Err(Layout {
            title: "Login".to_string(),
            lang: None,
            content: LoginPage {
                email: Some(login_form.email),
                error: Some("Invalid username or password".to_string()),
            },
        })
    }
}
