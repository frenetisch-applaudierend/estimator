use std::{cell::RefCell, rc::Rc};

use ui::{
    auth::{AuthenticationHandler, User},
    util::ServiceProp,
    ClientApp, ClientAppProps,
};

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    let auth_handler = ClientAuthenticationHandler::new();
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

struct ClientAuthenticationHandler {
    user: RefCell<Option<User>>,
}

impl ClientAuthenticationHandler {
    pub fn new() -> Self {
        Self {
            user: RefCell::new(None),
        }
    }
}

impl AuthenticationHandler for ClientAuthenticationHandler {
    fn get_user(&self) -> Option<User> {
        self.user.borrow().clone()
    }
}
