use std::future::Future;
use std::io;

use once_cell::sync::Lazy;

pub(crate) static DEFAULT_RUNTIME_SIZE: Lazy<usize> = Lazy::new(|| 0);

pub static NO_RUNTIME_NOTICE: &str = r#"No runtime configured for this platform, \
    features that requires task spawning can't be used. \
    Either compile with `target_arch = "wasm32", or enable the `tokio` feature."#;

#[inline(always)]
pub(super) fn spawn_local<F>(_f: F)
where
    F: Future<Output = ()> + 'static,
{
    panic!("{}", NO_RUNTIME_NOTICE);
}

#[derive(Debug, Clone)]
pub(crate) struct Runtime {}

impl Default for Runtime {
    fn default() -> Self {
        panic!("{}", NO_RUNTIME_NOTICE);
    }
}

impl Runtime {
    pub fn new(_size: usize) -> io::Result<Self> {
        panic!("{}", NO_RUNTIME_NOTICE);
    }

    pub(crate) async fn run_pinned<F, Fut>(&self, _create_task: F) -> Fut::Output
    where
        F: FnOnce() -> Fut,
        F: Send + 'static,
        Fut: Future + 'static,
        Fut::Output: Send + 'static,
    {
        panic!("{}", NO_RUNTIME_NOTICE);
    }
}
