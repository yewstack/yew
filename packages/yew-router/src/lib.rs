//! Provides routing faculties using the browser history API to build
//! Single Page Applications (SPAs) using [Yew web framework](https://yew.rs).
//!
//! # Usage
//!
//! ```rust
//! use yew::prelude::*;
//! use yew::functional::*;
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
//!     let history = use_history().unwrap();
//!
//!     let onclick_callback = Callback::from(move |_| history.push(Route::Home));
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
//!             <Switch<Route> render={Switch::render(switch)} />
//!         </BrowserRouter>
//!     }
//! }
//!
//! fn switch(routes: &Route) -> Html {
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
//! The router registers itself as a context provider and makes session history and location information
//! available via [`hooks`] or [`RouterScopeExt`](scope_ext::RouterScopeExt).
//!
//! # State
//!
//! The [`history`] API has a way access / store state associated with session history. Please
//! consule [`history.state()`](history::History::state) for detailed usage.

extern crate self as yew_router;

#[doc(hidden)]
#[path = "macro_helpers.rs"]
pub mod __macro;
pub mod components;
pub mod history;
pub mod hooks;
mod routable;
pub mod router;
pub mod scope_ext;
pub mod switch;
pub mod utils;

pub use routable::{AnyRoute, Routable};
pub use router::{BrowserRouter, Router};
pub use switch::{RenderFn, Switch};

pub mod prelude {
    //! Prelude module to be imported when working with `yew-router`.
    //!
    //! This module re-exports the frequently used types from the crate.

    pub use crate::components::{Link, Redirect};
    pub use crate::history::*;
    pub use crate::hooks::*;
    pub use crate::scope_ext::RouterScopeExt;
    #[doc(no_inline)]
    pub use crate::Routable;
    pub use crate::{BrowserRouter, Router};

    pub use crate::Switch;
}
