use std::collections::HashMap;

use yew::prelude::*;
use yew_router::{
    history::{AnyHistory, History, MemoryHistory},
    prelude::*,
};

use crate::{auth::AuthenticationHandler, pages};
use crate::{
    auth::{AuthenticationProvider, Protected},
    util::ServiceProp,
};

#[derive(Properties, PartialEq)]
pub struct ServerAppProps {
    pub url: AttrValue,
    pub query: HashMap<String, String>,
    pub auth_handler: ServiceProp<dyn AuthenticationHandler>,
}

#[function_component]
pub fn ServerApp(props: &ServerAppProps) -> Html {
    let history = AnyHistory::from(MemoryHistory::new());
    history
        .push_with_query(&*props.url, &props.query)
        .expect("Could not create history on server");

    html! {
        <AuthenticationProvider handler={props.auth_handler.clone()}>
            <Router history={history}>
                <AppContent />
            </Router>
        </AuthenticationProvider>
    }
}

#[derive(Properties, PartialEq)]
pub struct ClientAppProps {
    pub auth_handler: ServiceProp<dyn AuthenticationHandler>,
}

#[function_component]
pub fn ClientApp(props: &ClientAppProps) -> Html {
    html! {
        <AuthenticationProvider handler={props.auth_handler.clone()}>
            <BrowserRouter>
                <AppContent />
            </BrowserRouter>
        </AuthenticationProvider>
    }
}

#[function_component]
fn AppContent() -> Html {
    html! {
      <main>
        <Switch<Route> render={switch} />
      </main>
    }
}

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Main,

    #[at("/auth/login")]
    Login,

    #[at("/sub")]
    Sub,

    #[not_found]
    #[at("/error/404")]
    NotFound,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Main => html! { <pages::Main /> },
        Route::Login => html! { <pages::auth::Login /> },
        Route::Sub => html! { <Protected><pages::Sub /></Protected> },
        Route::NotFound => html! { "Page not found" },
    }
}
