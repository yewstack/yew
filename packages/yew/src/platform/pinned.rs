//! Task synchronisation primitives for pinned tasks.
//!
//! This module provides the following task synchronisation mechanisms for `!Send` futures:
//!
//! - [`Barrier`]: Ensures multiple tasks to wait until all tasks have reached a point in the
//! program before continuing execution of all together.
//! - [`RwLock`]: Provides a mutual exclusion mechanism which allows multiple readers at the same
//! time, while allowing only one writer at a time.
//! - [`mpsc`]: A channel that supports sending multiple values from multiple producers to a single
//! receiver.
//! - [`oneshot`]: A channel to send one single value from a producer to a receiver.

#[doc(inline)]
pub use pinned::*;
