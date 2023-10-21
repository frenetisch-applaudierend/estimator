use std::rc::Rc;

use ui::{
    auth::{AuthenticationHandler, User},
    util::ServiceProp,
    ClientApp, ClientAppProps,
};

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    let auth_handler = ClientAuthenticationHandler;
    let props = ClientAppProps {
        auth_handler: ServiceProp(Rc::new(auth_handler)),
    };
    let renderer = yew::Renderer::<ClientApp>::with_props(props);

    if cfg!(profile = "release") {
        renderer.hydrate();
    } else {
        renderer.render();
    }
}

struct ClientAuthenticationHandler;

impl AuthenticationHandler for ClientAuthenticationHandler {
    fn get_user(&self) -> Option<User> {
        todo!()
    }

    fn set_user(&self, user: Option<User>) {
        todo!()
    }
}
