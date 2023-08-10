//! This module provides task agent implementation.

mod hooks;
mod provider;

#[doc(inline)]
pub use gloo_worker::oneshot::{Oneshot, OneshotBridge, OneshotRegistrar, OneshotSpawner};
pub use hooks::{use_bridge_oneshot, UseBridgeOneshotHandle};
pub use provider::OneshotProvider;
pub(crate) use provider::OneshotProviderState;
