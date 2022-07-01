//! Utilities for bridging time and tasks.

use std::time::Duration;

use futures::Stream;

use crate::platform::imp::time as imp;

/// Waits until duration has elapsed.
///
/// # Panic
///
/// On some platforms, if the prodvided duration cannot be converted to u32 in milliseconds, this
/// function will panic.
pub async fn sleep(dur: Duration) {
    imp::sleep(dur).await;
}

/// Creates a Stream that yields an item for after every period has elapsed.
///
/// # Panic
///
/// On some platforms, if the prodvided period cannot be converted to u32 in milliseconds, this
/// function will panic.
pub fn interval(period: Duration) -> impl Stream<Item = ()> {
    imp::interval(period)
}
