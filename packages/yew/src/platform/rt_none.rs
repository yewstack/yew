use std::future::Future;

#[inline(always)]
pub(super) fn spawn_local<F>(_f: F)
where
    F: Future<Output = ()> + 'static,
{
    panic!(
        r#"No runtime configured for this platform, features that requires task spawning can't be used.
                Either compile with `target_arch = "wasm32", or enable the `tokio` feature."#
    );
}

#[cfg(feature = "ssr")]
pub(crate) async fn run_pinned<F, Fut>(_create_task: F) -> Fut::Output
where
    F: FnOnce() -> Fut,
    F: Send + 'static,
    Fut: Future + 'static,
    Fut::Output: Send + 'static,
{
    panic!(
        r#"No runtime configured for this platform, features that requires task spawning can't be used.
                Either compile with `target_arch = "wasm32", or enable the `tokio` feature."#
    )
}
