//! This module contains the worker agent implementation.
//!
//! This is a low-level implementation that provides convenient helpers for gloo-workers.

mod hooks;
mod provider;

#[doc(inline)]
pub use gloo_worker::{
    HandlerId, Spawnable, Worker, WorkerBridge, WorkerDestroyHandle, WorkerScope, WorkerSpawner,
};
pub use hooks::{
    use_worker_bridge, use_worker_subscription, UseWorkerBridgeHandle, UseWorkerSubscriptionHandle,
};
pub use provider::{WorkerProvider, WorkerProviderProps};
