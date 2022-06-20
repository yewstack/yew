//! This module contains the worker agent implementation.
//!
//! This is a low-level implementation that wraps gloo-worker.

mod hooks;
mod provider;

#[doc(inline)]
pub use gloo_worker::{HandlerId, Worker, WorkerDestroyHandle, WorkerRegistrar, WorkerScope};
pub use hooks::{
    use_worker_bridge, use_worker_subscription, UseWorkerBridgeHandle, UseWorkerSubscriptionHandle,
};
pub(crate) use provider::WorkerProviderState;
pub use provider::{WorkerProvider, WorkerProviderProps};
