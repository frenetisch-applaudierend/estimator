use std::collections::HashMap;

use yew::prelude::*;
use yew_router::{
    history::{AnyHistory, History, MemoryHistory},
    prelude::*,
};

use crate::pages;

#[derive(Properties, PartialEq, Eq, Debug)]
pub struct ServerAppProps {
    pub url: AttrValue,
    pub query: HashMap<String, String>,
}

#[function_component]
pub fn ServerApp(props: &ServerAppProps) -> Html {
    let history = AnyHistory::from(MemoryHistory::new());
    history
        .push_with_query(&*props.url, &props.query)
        .expect("Could not create history on server");

    html! {
        <Router history={history}>
          <AppContent />
        </Router>
    }
}

#[function_component]
pub fn ClientApp() -> Html {
    html! {
        <BrowserRouter>
          <AppContent />
        </BrowserRouter>
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

    #[at("/sub")]
    Sub,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Main => html! { <pages::Main /> },
        Route::Sub => html! { <pages::Sub /> },
    }
}
