//! This module provides task agent implementation.

mod hooks;
mod provider;

#[doc(inline)]
pub use gloo_worker::oneshot::{Oneshot, OneshotBridge, OneshotRegistrar, OneshotSpawner};
pub use hooks::{use_oneshot_runner, UseOneshotRunnerHandle};
pub use provider::OneshotProvider;
pub(crate) use provider::OneshotProviderState;
/// A procedural macro to create oneshot agents.
pub use yew_agent_macro::oneshot;
