use crate::components::login_form::LoginForm;
use yew::prelude::*;

#[function_component(Login)]
pub fn login() -> Html {
    html! {
        <LoginForm />
    }
}