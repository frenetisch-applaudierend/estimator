use std::rc::Rc;

use yew::prelude::*;

use crate::util::ServiceProp;

pub trait AuthenticationHandler {
    fn get_user(&self) -> Option<User>;
    fn set_user(&self, user: Option<User>);
}

#[derive(Clone)]
pub struct AuthenticationContext {
    handler: Rc<dyn AuthenticationHandler>,
    force_update_handle: UseForceUpdateHandle,
}

impl AuthenticationContext {
    pub fn is_logged_in(&self) -> bool {
        self.handler.get_user().is_some()
    }

    pub fn current_user(&self) -> Option<User> {
        self.handler.get_user()
    }

    pub fn login(&self, user: User) {
        self.handler.set_user(Some(user));
        self.force_update_handle.force_update();
    }

    pub fn logout(&self) {
        self.handler.set_user(None);
        self.force_update_handle.force_update();
    }
}

impl PartialEq for AuthenticationContext {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.handler.as_ref(), other.handler.as_ref())
    }
}

impl std::fmt::Debug for AuthenticationContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AuthenticationContext").finish()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct User {
    pub email: String,
}

#[hook]
pub fn use_authentication() -> AuthenticationContext {
    use_context::<AuthenticationContext>().expect(
        "No AuthenticationContext found. Did you add an <AuthenticationProvider> in a parent node?",
    )
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub handler: ServiceProp<dyn AuthenticationHandler>,
    pub children: Html,
}

#[function_component]
pub fn AuthenticationProvider(props: &Props) -> Html {
    let trigger = use_force_update();
    let context = AuthenticationContext {
        handler: props.handler.0.clone(),
        force_update_handle: trigger,
    };

    html! {
        <ContextProvider<AuthenticationContext> {context}>
          { props.children.clone() }
        </ContextProvider<AuthenticationContext>>
    }
}
