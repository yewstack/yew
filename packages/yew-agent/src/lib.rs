//! This module contains Yew's web worker implementation.
//!
//! ## Types
//!
//! There're a couple kinds of agents:
//!
//! #### Oneshot
//!
//! A kind of agent that for each input, a single output is returned.
//!
//! #### Reactor
//!
//! A kind of agent that can send many inputs and receive many outputs over a single bridge.
//!
//! #### Worker
//!
//! The low-level implementation of agents that provides an actor model and communicates with
//! multiple bridges.
//!
//! ## Reachability
//!
//! When an agent is spawned, each agent is associated with a reachability.
//!
//! #### Private
//!
//! Each time a bridge is created, a new instance
//! of agent is spawned. This allows parallel computing between agents.
//!
//! #### Public
//!
//! Public agents are shared among all children of a provider.
//! Only 1 instance will be spawned for each public agents provider.
//!
//! ### Provider
//!
//! Each Agent requires a provider to provide communications and maintain bridges.
//! All hooks must be called within a provider.
//!
//! ## Communications with Agents
//!
//! Hooks provides means to communicate with agent instances.
//!
//! #### Bridge
//!
//! See: [`use_worker_bridge`](worker::use_worker_bridge),
//! [`use_reactor_bridge`](reactor::use_reactor_bridge)
//!
//! A bridge takes a callback to receive outputs from agents
//! and provides a handle to send inputs to agents.
//!
//! #### Subscription
//!
//! See: [`use_worker_subscription`](worker::use_worker_subscription),
//! [`use_reactor_subscription`](reactor::use_reactor_subscription)
//!
//! Similar to bridges, a subscription produces a handle to send inputs to agents. However, instead
//! of notifying the receiver with a callback, it collect all outputs into a slice.
//!
//! #### Runner
//!
//! See: [`use_oneshot_runner`](oneshot::use_oneshot_runner)
//!
//! Unlike other agents, oneshot bridges provide a `use_oneshot_runner` hook to execute oneshot
//! agents on demand.

#![deny(
    clippy::all,
    missing_docs,
    missing_debug_implementations,
    bare_trait_objects,
    anonymous_parameters,
    elided_lifetimes_in_paths
)]

extern crate self as yew_agent;

pub mod oneshot;
pub mod reactor;
pub mod worker;

#[doc(inline)]
pub use gloo_worker::{Bincode, Codec, Registrable, Spawnable};

mod reach;
pub mod scope_ext;

pub use reach::Reach;

mod utils;

#[doc(hidden)]
pub mod __vendored {
    pub use futures;
}

pub mod prelude {
    //! Prelude module to be imported when working with `yew-agent`.
    //!
    //! This module re-exports the frequently used types from the crate.
    pub use crate::oneshot::{oneshot, use_oneshot_runner, UseOneshotRunnerHandle};
    pub use crate::reach::Reach;
    pub use crate::reactor::{
        reactor, use_reactor_bridge, use_reactor_subscription, ReactorEvent, ReactorScope,
        UseReactorBridgeHandle, UseReactorSubscriptionHandle,
    };
    pub use crate::scope_ext::{AgentScopeExt, ReactorBridgeHandle, WorkerBridgeHandle};
    pub use crate::worker::{
        use_worker_bridge, use_worker_subscription, UseWorkerBridgeHandle,
        UseWorkerSubscriptionHandle, WorkerScope,
    };
    pub use crate::{Registrable, Spawnable};
}
