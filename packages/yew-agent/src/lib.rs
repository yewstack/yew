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

pub mod codec;
pub mod oneshot;
pub mod reactor;
pub mod worker;
pub use codec::{Bincode, Codec};
pub mod traits;
pub use traits::{Registrable, Spawnable};

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
