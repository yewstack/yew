//! This module provides io compatibility over browser tasks and other asyncio runtimes (e.g.:
//! tokio).

use std::future::Future;

#[cfg(feature = "ssr")]
pub(crate) mod sync;

#[cfg(not(any(feature = "tokio", target_arch = "wasm32")))]
#[path = "rt_null/mod.rs"]
mod imp;
#[cfg(all(not(target_arch = "wasm32"), feature = "tokio"))]
#[path = "rt_tokio/mod.rs"]
mod imp;
#[cfg(target_arch = "wasm32")]
#[path = "rt_wasm_bindgen/mod.rs"]
mod imp;

/// Spawns a task on current thread.
#[inline(always)]
pub fn spawn_local<F>(f: F)
where
    F: Future<Output = ()> + 'static,
{
    imp::spawn_local(f);
}

/// Runs a task with it pinned onto a local worker thread.
///
/// This can be used to execute non-Send futures without blocking the current thread.
///
/// It maintains an internal thread pool dedicated to executing local futures.
///
/// [`spawn_local`] is available with tasks executed with `run_pinned`.
#[inline(always)]
#[cfg(feature = "ssr")]
pub(crate) async fn run_pinned<F, Fut>(create_task: F) -> Fut::Output
where
    F: FnOnce() -> Fut,
    F: Send + 'static,
    Fut: Future + 'static,
    Fut::Output: Send + 'static,
{
    imp::run_pinned(create_task).await
}
