use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component]
pub fn Sub() -> Html {
    html! {
        <main>
          <h1>{"Hello, Sub!"}</h1>
          <Link<Route> to={Route::Main}>
            {"Go to main"}
          </Link<Route>>
        </main>
    }
}
