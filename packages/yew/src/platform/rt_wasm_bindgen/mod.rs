use std::future::Future;
use std::io;

pub(crate) mod time;

pub(crate) fn get_default_runtime_size() -> usize {
    0
}

pub(super) use wasm_bindgen_futures::spawn_local;

#[derive(Debug, Clone, Default)]
pub(crate) struct Runtime {}

impl Runtime {
    pub fn new(_size: usize) -> io::Result<Self> {
        Ok(Self {})
    }

    pub fn spawn_pinned<F, Fut>(&self, create_task: F)
    where
        F: FnOnce() -> Fut,
        F: Send + 'static,
        Fut: Future<Output = ()> + 'static,
    {
        spawn_local(create_task())
    }
}
