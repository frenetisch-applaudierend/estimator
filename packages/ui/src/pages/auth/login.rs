use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::{auth::use_authentication, Route};

#[function_component]
pub fn Login() -> Html {
    let nav = use_navigator().expect("Could not get navigator");
    let _auth = use_authentication();

    let onlogin = Callback::from(move |_| {
        log::info!("Logging in...");
        // auth.login(User {
        //     email: "markus.gasser@mailbox.org".to_string(),
        // });

        nav.replace(&Route::Main);
    });

    html! {
        <div>
            <h1>{ "Please log in" }</h1>
            <button onclick={onlogin}>{ "Log in" }</button>
        </div>
    }
}
