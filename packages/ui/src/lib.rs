use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct AppProps {
    pub title: String,
}

#[function_component]
pub fn App(props: &AppProps) -> Html {
    let fallback = html! {<div>{"Loading..."}</div>};

    let props = props.clone();

    html! {
        <Suspense {fallback}>
            <Content ..props />
        </Suspense>
    }
}

#[function_component]
fn Content(props: &AppProps) -> HtmlResult {
    let title = props.title.clone();
    let title = use_prepared_state!((), |_| -> String { title })?.unwrap();

    Ok(html! {
        <h1>{ &*title }</h1>
    })
}
