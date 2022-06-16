//! This module provides task agent implementation.

mod hooks;
mod provider;
mod traits;

pub use hooks::{use_memorized_task, use_task, UseTaskHandle};
pub use provider::TaskProvider;
pub use traits::{Task, TaskRegistrar};
