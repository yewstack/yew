#[cfg(feature = "ssr")]
use std::future::Future;

use once_cell::sync::Lazy;

pub(crate) static DEFAULT_RUNTIME_SIZE: Lazy<usize> = Lazy::new(|| 0);

pub(super) use wasm_bindgen_futures::spawn_local;

#[derive(Debug, Clone, Default)]
pub(crate) struct Runtime {}

impl Runtime {
    pub fn new(_size: usize) -> io::Result<Self> {
        Ok(Self {})
    }

    pub(crate) async fn run_pinned<F, Fut>(&self, create_task: F) -> Fut::Output
    where
        F: FnOnce() -> Fut,
        F: Send + 'static,
        Fut: Future + 'static,
        Fut::Output: Send + 'static,
    {
        create_task().await
    }
}
