//! This module contains the worker agent implementation.
//!
//! This is a low-level implementation that provides convenient helpers for gloo-workers.

mod hooks;
mod provider;

#[doc(inline)]
pub use gloo_worker::{
    HandlerId, Worker, WorkerBridge, WorkerDestroyHandle, WorkerRegistrar, WorkerScope,
    WorkerSpawner,
};
pub use hooks::{
    use_worker_bridge, use_worker_subscription, UseWorkerBridgeHandle, UseWorkerSubscriptionHandle,
};
pub(crate) use provider::WorkerProviderState;
pub use provider::{WorkerProvider, WorkerProviderProps};
