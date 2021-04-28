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
//!     #[at("/404")]
//!     NotFound,
//! }
//!
//! # #[function_component(Main)]
//! # fn app() -> Html {
//! let onclick_callback = Callback::from(|_| RouterService::push(Routes::Home, None));
//! html! {<button onclick=onclick_callback/>}
//! # }
//! ```
//!
//! # Internals
//!
//! The router keeps its own internal state which is used to store the current route and its associated data.
//! This allows the [Router] to be operated using the [service][RouterService] with an API that
//! isn't cumbersome to use. It also caches results of computations such as the query parameters of
//! [current route][CurrentRoute], making calls to [`CurrentRoute::query`][CurrentRoute::query()]
//! or [`CurrentRoute::route`][CurrentRoute::route()] fairly cheap.
//!
//! # State
//!
//! The browser history API allows users to state associated with the route. This crate does
//! not expose or make use of it. It is instead recommended that a state management library like
//! [yewdux](https://github.com/intendednull/yewdux) be used.

pub mod components;
mod routable;
pub mod router;
mod service;
pub mod utils;

pub use routable::Routable;
pub use router::{RcWrapper, Router};
pub use service::RouterService;

pub use yew_router_macro::Routable;

pub mod prelude {
    //! Prelude module to be imported when working with `yew-router`.
    //!
    //! This module re-exports the frequently used types from the crate.

    pub use crate::components::link::Link;
    pub use crate::Router;
    #[doc(no_inline)]
    pub use crate::{Routable, RouterService};
}
