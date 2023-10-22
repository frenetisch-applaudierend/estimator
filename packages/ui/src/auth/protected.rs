use yew::prelude::*;
use yew_router::prelude::*;

use crate::{auth::context::use_authentication, Route};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html,
}

#[function_component]
pub fn Protected(props: &Props) -> Html {
    let navigator = use_navigator();
    let auth = use_authentication();
    let user = auth.current_user();
    let is_logged_in = user.is_some();

    use_effect_with(is_logged_in, move |_| {
        if !is_logged_in {
            log::warn!("Not logged in");
            navigator.map(|nav| nav.push(&Route::Login));
        }
    });

    match user {
        Some(_) => html! {
            { props.children.clone() }
        },
        None => html! {
            { "You do not have permission to see this page" }
        },
    }
}
