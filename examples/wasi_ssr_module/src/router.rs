use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Portal,

    #[at("/t/:id")]
    Thread { id: String },

    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Portal => {
            html! { <h1>{"Hello"}</h1> }
        }
        Route::Thread { id } => {
            html! { <h1>{format!("Thread id {}", id)}</h1> }
        }
        Route::NotFound => {
            html! { <h1>{"Not found"}</h1> }
        }
    }
}
