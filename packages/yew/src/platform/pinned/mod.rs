//! Task synchronisation primitives for pinned tasks.
//!
//! This module provides task synchronisation for `!Send` futures.

pub mod mpsc;
pub mod oneshot;
