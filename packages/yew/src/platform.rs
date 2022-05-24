//! This module provides io compatibility over browser tasks and other asyncio runtimes (e.g.:
//! tokio).

use std::future::Future;

#[cfg(target_arch = "wasm32")]
mod arch {
    pub(super) use wasm_bindgen_futures::spawn_local;

    use super::*;

    pub(super) async fn run_pinned<F, Fut>(create_task: F) -> Fut::Output
    where
        F: FnOnce() -> Fut,
        F: Send + 'static,
        Fut: Future + 'static,
        Fut::Output: Send + 'static,
    {
        create_task().await
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod arch {
    use super::*;

    #[cfg(feature = "tokio")]
    pub(super) async fn run_pinned<F, Fut>(create_task: F) -> Fut::Output
    where
        F: FnOnce() -> Fut,
        F: Send + 'static,
        Fut: Future + 'static,
        Fut::Output: Send + 'static,
    {
        use once_cell::sync::Lazy;
        use tokio_util::task::LocalPoolHandle;

        static POOL_HANDLE: Lazy<LocalPoolHandle> =
            Lazy::new(|| LocalPoolHandle::new(num_cpus::get() * 2));

        POOL_HANDLE
            .spawn_pinned(create_task)
            .await
            .expect("future has panicked!")
    }

    #[inline(always)]
    pub(super) fn spawn_local<F>(f: F)
    where
        F: Future<Output = ()> + 'static,
    {
        #[cfg(feature = "tokio")]
        ::tokio::task::spawn_local(f);
        #[cfg(not(feature = "tokio"))]
        {
            let _ = f;
            panic!(
                r#"No runtime configured for this platform, features that requires task spawning can't be used.
                Either compile with `target_arch = "wasm32", or enable the `tokio` feature."#
            );
        }
    }

    #[cfg(not(feature = "tokio"))]
    pub(crate) async fn run_pinned<F, Fut>(create_task: F) -> Fut::Output
    where
        F: FnOnce() -> Fut,
        F: Send + 'static,
        Fut: Future + 'static,
        Fut::Output: Send + 'static,
    {
        let _ = create_task;

        panic!(
            r#"No runtime configured for this platform, features that requires task spawning can't be used.
                Either compile with `target_arch = "wasm32", or enable the `tokio` feature."#
        )
    }
}

/// Spawns a task on current thread.
#[inline(always)]
pub fn spawn_local<F>(f: F)
where
    F: Future<Output = ()> + 'static,
{
    arch::spawn_local(f);
}

/// Runs a task with it pinned onto a local worker thread.
///
/// This can be used to execute non-Send futures without blocking the current thread.
///
/// It maintains an internal thread pool dedicated to executing local futures.
///
/// [`spawn_local`] is available with tasks executed with `run_pinned`.
#[inline(always)]
pub async fn run_pinned<F, Fut>(create_task: F) -> Fut::Output
where
    F: FnOnce() -> Fut,
    F: Send + 'static,
    Fut: Future + 'static,
    Fut::Output: Send + 'static,
{
    arch::run_pinned(create_task).await
}
