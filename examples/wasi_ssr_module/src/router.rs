use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Portal,

    #[at("/t/{id}")]
    Thread { id: String },

    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    html! {
        match routes {
            Route::Portal => <h1>{"Hello"}</h1>,
            Route::Thread { id } => <h1>{format!("Thread id {}", id)}</h1>,
            Route::NotFound => <h1>{"Not found"}</h1>,
        }
    }
}
