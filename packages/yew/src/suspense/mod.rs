//! This module provides suspense support.

mod component;
mod hooks;
mod suspension;

#[cfg(feature = "csr")]
pub(crate) use component::{resume_suspension, suspend_suspension, DispatchSuspension};
pub use component::{Suspense, SuspenseProps};
pub use hooks::*;
pub use suspension::{Suspension, SuspensionHandle, SuspensionResult};
