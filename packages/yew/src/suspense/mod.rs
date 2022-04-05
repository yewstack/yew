//! This module provides suspense support.

mod component;
mod suspension;

#[cfg(any(feature = "csr", feature = "ssr"))]
pub(crate) use component::BaseSuspense;
pub use component::{Suspense, SuspenseProps};
pub use suspension::{Suspension, SuspensionHandle, SuspensionResult};
