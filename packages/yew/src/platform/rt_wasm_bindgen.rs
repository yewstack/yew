#[cfg(feature = "ssr")]
use std::future::Future;

pub(super) use wasm_bindgen_futures::spawn_local;

#[cfg(feature = "ssr")]
pub(crate) async fn run_pinned<F, Fut>(create_task: F) -> Fut::Output
where
    F: FnOnce() -> Fut,
    F: Send + 'static,
    Fut: Future + 'static,
    Fut::Output: Send + 'static,
{
    create_task().await
}
