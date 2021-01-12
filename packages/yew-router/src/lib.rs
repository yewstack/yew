#![recursion_limit = "128"]
//! Provides routing faculties for the Yew web framework.
//!
//! ## Contents
//! This crate consists of multiple types, some independently useful on their own,
//! that are used together to facilitate routing within the Yew framework.
//! Among them are:
//! * RouteService - Hooks into the History API and listens to `PopStateEvent`s to respond to users
//!   clicking the back/forwards buttons.
//! * RouteAgent - A singleton agent that owns a RouteService that provides an easy place for other
//!   components and agents to hook into it.
//! * Switch - A trait/derive macro that allows specification of how enums or structs can be constructed
//! from Routes.
//! * Router - A component connected to the RouteAgent, and is capable of resolving Routes to
//! Switch implementors, so you can use them to render Html.
//! * Route - A struct containing an the route string and state.
//! * RouteButton & RouteLink - Wrapper components around buttons and anchor tags respectively that
//!   allow users to change the route.
//!
//! ## State and Aliases
//! Because the History API allows you to store data along with a route string,
//! most types have at type parameter that allows you to specify which type is being stored.
//! As this behavior is uncommon, aliases using the unit type (`()`) are provided to remove the
//! need to specify the storage type you likely aren't using.
//!
//! If you want to store state using the history API, it is recommended that you generate your own
//! aliases using the `define_router_state` macro.
//! Give it a typename, and it will generate a module containing aliases and functions useful for
//! routing. If you specify your own router_state aliases and functions, you will want to disable
//! the `unit_alias` feature to prevent the default `()` aliases from showing up in the prelude.
//!
//! ## Features
//! This crate has some feature-flags that allow you to not include some parts in your compilation.
//! * "default" - Everything is included by default.
//! * "core" - The fully feature complete ("router", "components", "matchers"), but without
//!   unit_alias.
//! * "unit_alias" - If enabled, a module will be added to the route and expanded within the prelude
//! for aliases of Router types to their `()` variants.
//! * "router" - If enabled, the Router component and its dependent infrastructure (including
//!   "agent") will be included.
//! * "agent" - If enabled, the RouteAgent and its associated types will be included.
//! * "components" - If enabled, the accessory components will be made available.

#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_qualifications
)]
// This will break the project at some point, but it will break yew as well.
// It can be dealt with at the same time.
#![allow(macro_expanded_macro_exports_accessed_by_absolute_paths)]

pub use yew_router_route_parser;

#[macro_use]
mod alias;

#[cfg(feature = "service")]
pub mod service;

#[cfg(feature = "agent")]
pub mod agent;

pub mod route;

#[cfg(feature = "components")]
pub mod components;

#[cfg(feature = "router")]
pub mod router;

/// Prelude module that can be imported when working with the yew_router
pub mod prelude {
    pub use super::matcher::Captures;

    #[cfg(feature = "service")]
    pub use crate::route::RouteState;
    #[cfg(feature = "service")]
    pub use crate::service::RouteService;

    #[cfg(feature = "agent")]
    pub use crate::agent::RouteAgent;
    #[cfg(feature = "agent")]
    pub use crate::agent::RouteAgentBridge;
    #[cfg(feature = "agent")]
    pub use crate::agent::RouteAgentDispatcher;

    #[cfg(feature = "components")]
    pub use crate::components::RouterAnchor;
    #[cfg(feature = "components")]
    pub use crate::components::RouterButton;

    #[cfg(feature = "router")]
    pub use crate::router::Router;

    #[cfg(feature = "router")]
    pub use crate::router::RouterState;

    pub use crate::{
        route::Route,
        switch::{Routable, Switch},
    };
    pub use yew_router_macro::Switch;
}

pub use alias::*;

pub mod matcher;

pub use matcher::Captures;

#[cfg(feature = "service")]
pub use crate::route::RouteState;
#[cfg(feature = "router")]
pub use crate::router::RouterState;

pub mod switch;
pub use switch::Switch;
pub use yew_router_macro::Switch;
