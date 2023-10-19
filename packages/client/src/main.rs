use ui::{App, AppProps};

fn main() {
    let props = AppProps {
        title: "Hello, Client".to_string(),
    };
    yew::Renderer::<App>::with_props(props).hydrate();
}
