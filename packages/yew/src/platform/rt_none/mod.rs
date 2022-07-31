use std::future::Future;

pub(crate) mod time;

static NO_RUNTIME_NOTICE: &str = r#"No runtime configured for this platform, \
    features that requires a runtime can't be used. \
    Either compile with `target_arch = "wasm32", or enable the `tokio` feature."#;

fn panic_no_runtime() -> ! {
    panic!("{}", NO_RUNTIME_NOTICE);
}

#[inline(always)]
pub(super) fn spawn_local<F>(_f: F)
where
    F: Future<Output = ()> + 'static,
{
    panic_no_runtime();
}

#[cfg(feature = "ssr")]
pub(crate) async fn run_pinned<F, Fut>(_create_task: F) -> Fut::Output
where
    F: FnOnce() -> Fut,
    F: Send + 'static,
    Fut: Future + 'static,
    Fut::Output: Send + 'static,
{
    panic_no_runtime();
}
