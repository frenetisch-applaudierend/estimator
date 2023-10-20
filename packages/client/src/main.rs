use ui::ClientApp;

fn main() {
    let renderer = yew::Renderer::<ClientApp>::new();

    if cfg!(profile = "release") {
        renderer.hydrate();
    } else {
        renderer.render();
    }
}
