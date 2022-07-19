//! Utilities for bridging time and tasks.

use std::future::Future;
use std::time::Duration;

use futures::stream::Stream;

use crate::platform::imp::time as imp;

/// Waits until duration has elapsed.
///
/// # Panics
///
/// On some platforms, if the prodvided duration cannot be converted to u32 in milliseconds, this
/// function will panic.
#[inline(always)]
pub fn sleep(dur: Duration) -> impl Future<Output = ()> {
    imp::sleep(dur)
}

/// Creates a Stream that yields an item after every period has elapsed.
///
/// # Panics
///
/// On some platforms, if the prodvided period cannot be converted to u32 in milliseconds, this
/// function will panic.
#[inline(always)]
pub fn interval(period: Duration) -> impl Stream<Item = ()> {
    imp::interval(period)
}
