use yew::prelude::*;
use yew_router::prelude::*;

use crate::{auth::context::use_authentication, Route};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html,
}

#[function_component]
pub fn Protected(props: &Props) -> Html {
    let auth = use_authentication();
    let user = auth.current_user();

    match user {
        Some(_) => html! {
            { props.children.clone() }
        },
        None => html! {
            <Redirect<Route> to={Route::Login} />
        },
    }
}
