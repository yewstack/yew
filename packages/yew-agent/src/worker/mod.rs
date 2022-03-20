//! This module contains the worker agent implementation.

mod hooks;
mod provider;

pub use hooks::{
    use_worker_bridge, use_worker_subscription, UseWorkerBridgeHandle, UseWorkerSubscriptionHandle,
};
pub use provider::{WorkerProvider, WorkerProviderProps};

#[doc(inline)]
pub use gloo_worker::{HandlerId, Spawnable, Worker, WorkerBridge, WorkerScope, WorkerSpawner};
