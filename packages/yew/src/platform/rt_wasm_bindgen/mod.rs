use std::future::Future;
use std::io;
use std::marker::PhantomData;

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

#[derive(Debug, Clone)]
pub(crate) struct LocalHandle {
    // This type is not send or sync.
    _marker: PhantomData<*const ()>,
}

impl LocalHandle {
    pub fn try_current() -> Option<Self> {
        Some(Self {
            _marker: PhantomData,
        })
    }

    pub fn current() -> Self {
        Self {
            _marker: PhantomData,
        }
    }

    pub fn spawn_local<F>(&self, f: F)
    where
        F: Future<Output = ()> + 'static,
    {
        spawn_local(f);
    }
}
