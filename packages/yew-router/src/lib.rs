//! Provides routing faculties using the browser history API to build
//! Single Page Applications (SPAs) using [Yew web framework](https://yew.rs).
//!
//! # Usage
//!
//! ```rust
//! use yew::functional::*;
//! use yew::prelude::*;
//! use yew_router::prelude::*;
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
//! #[function_component(Secure)]
//! fn secure() -> Html {
//!     let navigator = use_navigator().unwrap();
//!
//!     let onclick_callback = Callback::from(move |_| navigator.push(&Route::Home));
//!     html! {
//!         <div>
//!             <h1>{ "Secure" }</h1>
//!             <button onclick={onclick_callback}>{ "Go Home" }</button>
//!         </div>
//!     }
//! }
//!
//! #[function_component(Main)]
//! fn app() -> Html {
//!     html! {
//!         <BrowserRouter>
//!             <Switch<Route> render={switch} />
//!         </BrowserRouter>
//!     }
//! }
//!
//! fn switch(routes: Route) -> Html {
//!     match routes {
//!         Route::Home => html! { <h1>{ "Home" }</h1> },
//!         Route::Secure => html! {
//!             <Secure />
//!         },
//!         Route::NotFound => html! { <h1>{ "404" }</h1> },
//!     }
//! }
//! ```
//!
//! # Internals
//!
//! The router registers itself as a context provider and makes location information and navigator
//! available via [`hooks`] or [`RouterScopeExt`](scope_ext::RouterScopeExt).
//!
//! # State
//!
//! The [`Location`](gloo::history::Location) API has a way to access / store state associated with
//! session history. Please consult [`location.state()`](crate::history::Location::state) for
//! detailed usage.

extern crate self as yew_router;

#[doc(hidden)]
#[path = "macro_helpers.rs"]
pub mod __macro;
pub mod components;
pub mod hooks;
pub mod navigator;
mod routable;
pub mod router;
pub mod scope_ext;
pub mod switch;
pub mod utils;

pub use routable::{AnyRoute, Routable};
pub use router::{BrowserRouter, HashRouter, Router};
pub use switch::Switch;

pub mod history {
    //! A module that provides universal session history and location information.

    pub use gloo::history::{
        AnyHistory, BrowserHistory, HashHistory, History, HistoryError, HistoryResult, Location,
        MemoryHistory,
    };
}

pub mod prelude {
    //! Prelude module to be imported when working with `yew-router`.
    //!
    //! This module re-exports the frequently used types from the crate.

    pub use crate::components::{Link, Redirect};
    pub use crate::history::Location;
    pub use crate::hooks::*;
    pub use crate::navigator::{NavigationError, NavigationResult, Navigator};
    pub use crate::scope_ext::{LocationHandle, NavigatorHandle, RouterScopeExt};
    #[doc(no_inline)]
    pub use crate::Routable;
    pub use crate::{BrowserRouter, HashRouter, Router, Switch};
}
