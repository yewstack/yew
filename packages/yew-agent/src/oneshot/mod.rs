//! This module provides task agent implementation.

mod bridge;
mod hooks;
mod provider;
mod registrar;
mod spawner;
mod traits;
mod worker;

pub use bridge::OneshotBridge;
pub use hooks::{use_oneshot_runner, UseOneshotRunnerHandle};
pub use provider::OneshotProvider;
pub(crate) use provider::OneshotProviderState;
pub use registrar::OneshotRegistrar;
pub use spawner::OneshotSpawner;
pub use traits::Oneshot;
/// A procedural macro to create oneshot agents.
pub use yew_agent_macro::oneshot;
