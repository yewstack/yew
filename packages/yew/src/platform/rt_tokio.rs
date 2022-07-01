use std::future::Future;

#[cfg(feature = "ssr")]
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
    tokio::task::spawn_local(f);
}
