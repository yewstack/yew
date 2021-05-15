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
//! enum Routes {
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
//!     <Router<Routes> render=Router::render(switch) />
//! }
//! # }
//!
//! fn switch(routes: Routes) -> Html {
//!     let onclick_callback = Callback::from(|_| service::push(Routes::Home));
//!     match routes {
//!         Routes::Home => html! { <h1>{ "Home" }</h1> },
//!         Routes::Secure => html! {
//!             <div>
//!                 <h1>{ "Secure" }</h1>
//!                 <button onclick=onclick_callback>{ "Go Home" }</button>
//!             </div>
//!         },
//!         Routes::NotFound => html! { <h1>{ "404" }</h1> },
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

pub mod components;
mod routable;
pub mod router;
pub mod service;
pub mod utils;

pub use routable::Routable;
pub use router::{RenderFn, Router};

pub mod prelude {
    //! Prelude module to be imported when working with `yew-router`.
    //!
    //! This module re-exports the frequently used types from the crate.

    pub use crate::components::Link;
    #[doc(no_inline)]
    pub use crate::Routable;
    pub use crate::{service, Router};
}
