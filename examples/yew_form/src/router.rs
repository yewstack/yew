use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::login::Login;
use crate::pages::error::Error;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/error")]
    Error,
    #[at("/")]
    Login,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Login => html! { <Login /> },
        Route::Error => html! { <Error /> },
    }
}