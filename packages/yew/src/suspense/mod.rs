//! This module provides suspense support.

mod component;
mod suspension;

pub use component::Suspense;
pub use suspension::{Suspension, SuspensionHandle, SuspensionResult};
