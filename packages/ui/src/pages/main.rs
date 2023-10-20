use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component]
pub fn Main() -> Html {
    html! {
        <main>
          <h1>{"Hello, Main!"}</h1>
          <Link<Route> to={Route::Sub}>
            {"Go to sub"}
          </Link<Route>>
        </main>
    }
}
