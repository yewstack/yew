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
//! ### Worker
//!
//! The low-level implementation of agents that provides an actor model and communicates with
//! bridges.
//!
//! ## Reachability
//!
//! Agents needs to be spawned with a reachability.
//! There're currently 2 kinds of reachability:
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

pub mod station;
pub mod task;
pub mod worker;

mod reach;

pub use reach::Reach;
