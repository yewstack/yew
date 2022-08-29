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
use std::io::Result;
use std::marker::PhantomData;

#[cfg(feature = "ssr")]
pub(crate) mod fmt;

pub mod pinned;
pub mod time;

#[cfg(target_arch = "wasm32")]
#[path = "rt_wasm_bindgen/mod.rs"]
mod imp;
#[cfg(all(not(target_arch = "wasm32"), feature = "tokio"))]
#[path = "rt_tokio/mod.rs"]
mod imp;
#[cfg(all(not(target_arch = "wasm32"), not(feature = "tokio")))]
#[path = "rt_none/mod.rs"]
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

/// A Runtime Builder.
#[derive(Debug)]
pub struct RuntimeBuilder {
    worker_threads: usize,
}

impl Default for RuntimeBuilder {
    fn default() -> Self {
        Self {
            worker_threads: imp::get_default_runtime_size(),
        }
    }
}

impl RuntimeBuilder {
    /// Creates a new Runtime Builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the number of worker threads the Runtime will use.
    ///
    /// # Default
    ///
    /// The default number of worker threads is the number of available logical CPU cores.
    ///
    /// # Note
    ///
    /// This setting has no effect if current platform has no thread support (e.g.: WebAssembly).
    pub fn worker_threads(&mut self, val: usize) -> &mut Self {
        self.worker_threads = val;

        self
    }

    /// Creates a Runtime.
    pub fn build(&mut self) -> Result<Runtime> {
        Ok(Runtime {
            inner: imp::Runtime::new(self.worker_threads)?,
        })
    }
}

/// The Yew Runtime.
#[derive(Debug, Clone, Default)]
pub struct Runtime {
    inner: imp::Runtime,
}

impl Runtime {
    /// Creates a runtime Builder.
    pub fn builder() -> RuntimeBuilder {
        RuntimeBuilder::new()
    }

    /// Spawns a task with it pinned to a worker thread.
    ///
    /// This can be used to execute non-Send futures without blocking the current thread.
    ///
    /// [`spawn_local`] is available with tasks executed with `spawn_pinned`.
    pub fn spawn_pinned<F, Fut>(&self, create_task: F)
    where
        F: FnOnce() -> Fut,
        F: Send + 'static,
        Fut: Future<Output = ()> + 'static,
    {
        self.inner.spawn_pinned(create_task);
    }
}

/// A Local Runtime Handle.
///
/// This type can be used to acquire a runtime handle to spawn local tasks.
#[derive(Debug, Clone)]
pub struct LocalHandle {
    inner: imp::LocalHandle,
    // This type is not send or sync.
    _marker: PhantomData<*const ()>,
}

impl LocalHandle {
    /// Creates a Handle to current Runtime worker.
    ///
    /// # Panics
    ///
    /// This method will panic if not called within Yew Runtime.
    pub fn current() -> Self {
        let inner = imp::LocalHandle::current();

        Self {
            inner,
            _marker: PhantomData,
        }
    }

    /// Creates a Handle to current Runtime worker.
    ///
    /// This methods will return `None` if called from outside Yew Runtime.
    pub fn try_current() -> Option<Self> {
        let inner = imp::LocalHandle::try_current()?;

        Some(Self {
            inner,
            _marker: PhantomData,
        })
    }

    /// Spawns a Future with current Runtime worker.
    pub fn spawn_local<F>(&self, f: F)
    where
        F: Future<Output = ()> + 'static,
    {
        self.inner.spawn_local(f);
    }
}
