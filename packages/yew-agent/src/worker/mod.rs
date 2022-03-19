//! This module contains the worker agent implementation.

mod hook;
mod provider;

pub use hook::{use_bridge, UseBridgeHandle};
pub use provider::{WorkerProvider, WorkerProviderProps};

#[doc(inline)]
pub use gloo_worker::{Bridge, HandlerId, Spawnable, Spawner, Worker, WorkerScope};
