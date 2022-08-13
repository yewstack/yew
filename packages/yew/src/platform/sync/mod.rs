//! A module that provides task synchronisation primitives.

#[doc(inline)]
pub use tokio::sync::oneshot;
pub mod mpsc;
