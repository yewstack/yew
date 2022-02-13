//! This module provides suspense support.

mod component;
mod suspension;

pub(crate) use component::BaseSuspense;
pub use component::Suspense;
pub use suspension::{Suspension, SuspensionHandle, SuspensionResult};
