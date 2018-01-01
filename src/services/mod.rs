//! This module is a container of servies to interact with the external resources.
//!
//! It carries a similar role as subscriptions in Elm, but can be used directly
//! from the `update` method.

pub mod timeout;
pub mod interval;
pub mod storage;
pub mod dialog;
pub mod console;
pub mod fetch;
pub mod websocket;

use std::time::Duration;

/// An universal interface to service's routine. At least could be canceled.
pub trait Task {
    /// Cancel current service's routine.
    fn cancel(&mut self);
}

#[doc(hidden)]
fn to_ms(duration: Duration) -> u32 {
    let ms = duration.subsec_nanos() / 1_000_000;
    ms + duration.as_secs() as u32 * 1000
}
