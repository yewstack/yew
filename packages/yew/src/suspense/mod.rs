//! This module provides suspense support.

mod component;
mod hooks;
mod suspension;

#[cfg(any(feature = "csr", feature = "ssr"))]
pub(crate) use component::BaseSuspense;
pub use component::{Suspense, SuspenseProps};
pub use hooks::*;
pub use suspension::{Suspension, SuspensionHandle, SuspensionResult};
