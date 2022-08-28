use std::future::Future;
use std::io;
use std::marker::PhantomData;

pub(crate) mod time;

pub(crate) fn get_default_runtime_size() -> usize {
    0
}

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

#[derive(Debug, Clone)]
pub(crate) struct Runtime {}

impl Default for Runtime {
    fn default() -> Self {
        panic_no_runtime();
    }
}

impl Runtime {
    pub fn new(_size: usize) -> io::Result<Self> {
        panic_no_runtime();
    }

    pub fn spawn_pinned<F, Fut>(&self, _create_task: F)
    where
        F: FnOnce() -> Fut,
        F: Send + 'static,
        Fut: Future<Output = ()> + 'static,
    {
        panic_no_runtime();
    }
}

#[derive(Debug, Clone)]
pub(crate) struct LocalHandle {
    // This type is not send or sync.
    _marker: PhantomData<*const ()>,
}

impl LocalHandle {
    pub fn try_current() -> Option<Self> {
        panic_no_runtime();
    }

    pub fn current() -> Self {
        panic_no_runtime();
    }

    pub fn spawn_local<F>(&self, _f: F)
    where
        F: Future<Output = ()> + 'static,
    {
        panic_no_runtime();
    }
}
