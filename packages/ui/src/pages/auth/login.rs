use yew::prelude::*;

use crate::auth::use_authentication;

#[function_component]
pub fn Login() -> Html {
    let auth = use_authentication();

    let onlogin = Callback::from(move |_| {
        log::info!("Logging in...");
    });

    html! {
        <div>
            <h1>{ "Please log in" }</h1>
            <button onclick={onlogin}>{ "Log in" }</button>
        </div>
    }
}
