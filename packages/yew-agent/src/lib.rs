#![doc = include_str!("../README.md")]
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
    pub use crate::oneshot::{UseOneshotRunnerHandle, oneshot, use_oneshot_runner};
    pub use crate::reach::Reach;
    pub use crate::reactor::{
        ReactorEvent, ReactorScope, UseReactorBridgeHandle, UseReactorSubscriptionHandle, reactor,
        use_reactor_bridge, use_reactor_subscription,
    };
    pub use crate::scope_ext::{AgentScopeExt, ReactorBridgeHandle, WorkerBridgeHandle};
    pub use crate::worker::{
        UseWorkerBridgeHandle, UseWorkerSubscriptionHandle, WorkerScope, use_worker_bridge,
        use_worker_subscription,
    };
    pub use crate::{Registrable, Spawnable};
}
