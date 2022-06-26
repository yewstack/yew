//! Compatibility between JavaScript Runtime and Native Runtimes.
//!
//! When designing components and libraries that works on both WebAssembly targets backed by
//! JavaScript Runtime and non-WebAssembly targets with Native Runtimes. Developers usually face
//! challenges that requires applying multiple feature flags throughout their application:
//!
//! 1. Select I/O and timers that works with the target runtime.
//! 2. Native Runtimes usually require `Send` futures and WebAssembly usually use `!Send`
//! primitives for better performance during Client-side Rendering.
//!
//! To alleviate these issues, Yew implements a single-threaded runtime that executes `?Send`
//! (`Send` or `!Send`) futures. When your application starts with `yew::Renderer` or is rendered by
//! `yew::ServerRenderer`, it is executed within the Yew runtime. On systems with multi-threading
//! support, it spawns multiple independent runtimes in a worker pool proportional to the CPU
//! core number. The renderer will randomly select a worker thread from the internal pool. All tasks
//! spawned with `spawn_local` in the application will run on the same thread as the
//! rendering thread the renderer has selected. When the renderer runs in a WebAssembly target, all
//! tasks will be scheduled on the main thread.
//!
//! This runtime is designed in favour of IO-bounded workload with similar runtime cost. It produces
//! better performance by pinning tasks to a single worker thread. However, this means that if a
//! worker thread is back-logged, other threads will not be able to "help" by running tasks
//! scheduled on the busy thread. When you have a CPU-bounded task where CPU time is significantly
//! more expensive than rendering tasks, it should be spawned with a dedicated thread or
//! `yew-agent` and communicates with the application using channels or agent bridges.
//!
//! # Runtime Backend
//!
//! Yew runtime is implemented with different runtimes depending on the target platform and can use
//! all features (timers / IO / task synchronisation) from the selected native runtime:
//!
//! - `wasm-bindgen-futures` (WebAssembly targets)
//! - `tokio` (non-WebAssembly targets)
//!
//! # Compatibility with other async runtimes
//!
//! Yew's ServerRenderer can also be executed in applications using other async runtimes(e.g.:
//! `async-std`). Rendering tasks will enter Yew runtime and be executed with `tokio`. When the
//! rendering task finishes, the result is returned to the original runtime. This process is
//! transparent to the future that executes the renderer. The Yew application still needs to use
//! `tokio`'s timer, IO and task synchronisation primitives.

use std::future::Future;

#[cfg(feature = "ssr")]
pub(crate) mod io;

pub mod sync;

#[cfg(not(any(feature = "tokio", target_arch = "wasm32")))]
#[path = "rt_none.rs"]
mod imp;
#[cfg(all(not(target_arch = "wasm32"), feature = "tokio"))]
#[path = "rt_tokio.rs"]
mod imp;
#[cfg(target_arch = "wasm32")]
#[path = "rt_wasm_bindgen.rs"]
mod imp;

/// Spawns a task on current thread.
///
/// # Panics
///
/// This function will panic when not being executed from within a Yew Application.
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
