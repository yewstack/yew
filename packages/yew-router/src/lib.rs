//! Provides routing faculties using the browser history API to build
//! Single Page Applications (SPAs) using [Yew web framework](https://yew.rs).
//!
//! # Usage
//!
//! ```rust
//! # use yew::prelude::*;
//! # use yew_functional::*;
//! # use yew_router::prelude::*;
//!
//! #[derive(Debug, Clone, Copy, PartialEq, Routable)]
//! enum Route {
//!     #[at("/")]
//!     Home,
//!     #[at("/secure")]
//!     Secure,
//!     #[not_found]
//!     #[at("/404")]
//!     NotFound,
//! }
//!
//! # #[function_component(Main)]
//! # fn app() -> Html {
//! html! {
//!     <Router<Route> render=Router::render(switch) />
//! }
//! # }
//!
//! fn switch(routes: &Route) -> Html {
//!     let onclick_callback = Callback::from(|_| yew_router::push_route(Route::Home));
//!     match routes {
//!         Route::Home => html! { <h1>{ "Home" }</h1> },
//!         Route::Secure => html! {
//!             <div>
//!                 <h1>{ "Secure" }</h1>
//!                 <button onclick=onclick_callback>{ "Go Home" }</button>
//!             </div>
//!         },
//!         Route::NotFound => html! { <h1>{ "404" }</h1> },
//!     }
//! }
//! ```
//!
//! # Internals
//!
//! The router keeps its own internal state which is used to store the current route and its associated data.
//! This allows the [Router] to be operated using the [service] with an API that
//! isn't cumbersome to use.
//!
//! # State
//!
//! The browser history API allows users to state associated with the route. This crate does
//! not expose or make use of it. It is instead recommended that a state management library like
//! [yewdux](https://github.com/intendednull/yewdux) be used.

#[doc(hidden)]
#[path = "macro_helpers.rs"]
pub mod __macro;
pub mod components;
mod routable;
pub mod router;
mod service;
pub mod utils;

pub use service::*;

pub use routable::Routable;
pub use router::{RenderFn, Router};

pub mod prelude {
    //! Prelude module to be imported when working with `yew-router`.
    //!
    //! This module re-exports the frequently used types from the crate.

    pub use crate::components::Link;
    #[doc(no_inline)]
    pub use crate::Routable;
    pub use crate::Router;
}
