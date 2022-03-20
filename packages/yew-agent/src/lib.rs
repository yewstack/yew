//! This module contains Yew's web worker implementation.
//!
//! ## Types
//!
//! There're a couple kinds of agents:
//!
//! ### Task
//!
//! A kind of agent that for each input, a single output is expected.
//!
//! ### Reactor
//!
//! A kind of agent that can send many inputs and receive many outputs over a single bridge.
//!
//! ### Station
//!
//! A kind of agent that can receive many inputs and send many outputs over multiple bridges.
//!
//! ### Worker
//!
//! The low-level implementation of agents that provides an actor model and communicates with
//! bridges.
//!
//! ## Reachability
//!
//! When an agent is spawned, each agent is associated with a reachability.
//!
//! ### Private
//!
//! Each time a bridge is created with the `use_bridge`, a new instance
//! of agent is spawned. This allows parallel computing between agents.
//!
//! ### Public
//!
//! Public agents are shared among all children of a [WorkerProvider].
//! Only 1 instance will be spawned for each public agents provider.

#![deny(
    clippy::all,
    missing_docs,
    missing_debug_implementations,
    bare_trait_objects,
    anonymous_parameters,
    elided_lifetimes_in_paths
)]

extern crate self as yew_agent;

pub mod reactor;
pub mod station;
pub mod task;
pub mod worker;

/// A procedural macro to create station agents.
pub use yew_agent_macro::station;

mod reach;

pub use reach::Reach;

#[doc(hidden)]
pub mod __vendored {
    pub use wasm_bindgen_futures;
}
