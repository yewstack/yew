//! This module provides io compatibility over browser tasks and other asyncio runtimes (e.g.:
//! tokio).
//!
//! Yew implements a single-threaded runtime that executes `!Send` futures. When your application
//! starts with `yew::Renderer` or is rendered by `yew::ServerRenderer`, it is executed within the
//! Yew runtime. The renderer will select a worker thread from the internal worker
//! pool of Yew runtime. All tasks spawned with `spawn_local` will run on the same worker thread as
//! the rendering thread the renderer has selected. When the renderer runs in a WebAssembly target,
//! all tasks will be scheduled on the main thread.
//!
//! Yew runtime is implemented with native runtimes depending on the target platform and can use
//! all features (timers / IO / task synchronisation) from the selected native runtime:
//!
//! - `wasm-bindgen-futures` (WebAssembly targets)
//! - `tokio` (non-WebAssembly targets)
//!
//! Yew runtime alleviates the implementation requirement of `Send` futures when running with
//! multi-threaded runtimes like `tokio` and `!Send` futures on WebAssembly platforms and produces
//! good performance when the workload is IO-bounded and have similar runtime cost. When you have an
//! expensive CPU-bounded task, it should be spawned with a `Send`-aware spawning mechanism provided
//! by the native runtime, `std::thread::spawn` or `yew-agent` and communicates with the application
//! using channels or agent bridges.
//!
//! Yew's ServerRenderer can also be executed in applications using the `async-std` runtime.
//! Rendering tasks will enter Yew runtime and be executed with `tokio`. When the rendering task
//! finishes, the result is returned to the `async-std` runtime. This process is transparent to the
//! future that executes the renderer. The Yew application still needs to use `tokio`'s timer, IO
//! and task synchronisation primitives.

use std::future::Future;

#[cfg(feature = "ssr")]
pub(crate) mod io;

#[cfg(feature = "ssr")]
pub(crate) mod sync;

#[cfg(not(any(feature = "tokio", target_arch = "wasm32")))]
#[path = "rt_none/mod.rs"]
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
