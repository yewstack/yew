//! This module contains a scheduler.

use crate::platform::spawn_local;

/// Push a generic [Runnable] to be executed
#[inline(always)]
pub fn push<F>(runnable: F)
where
    F: FnOnce() + 'static,
{
    spawn_local(async move {
        runnable();
    });
}
