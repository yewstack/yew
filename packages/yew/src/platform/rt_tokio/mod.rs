use std::future::Future;
use std::io;

use once_cell::sync::Lazy;
use tokio::runtime::{Builder as TokioRuntimeBuilder, Runtime as TokioRuntime};
use tokio::task::LocalSet;
use tokio_util::task::LocalPoolHandle;

pub(crate) mod time;

pub(crate) fn get_default_runtime_size() -> usize {
    pub(crate) static DEFAULT_RUNTIME_SIZE: Lazy<usize> = Lazy::new(num_cpus::get);

    *DEFAULT_RUNTIME_SIZE
}

#[inline(always)]
pub(super) fn spawn_local<F>(f: F)
where
    F: Future<Output = ()> + 'static,
{
    tokio::task::spawn_local(f);
}

#[derive(Debug, Clone)]
pub(crate) struct Runtime {
    pool: LocalPoolHandle,
}

impl Default for Runtime {
    fn default() -> Self {
        static DEFAULT_RT: Lazy<Runtime> = Lazy::new(|| {
            Runtime::new(get_default_runtime_size()).expect("failed to create runtime.")
        });

        DEFAULT_RT.clone()
    }
}

impl Runtime {
    pub fn new(size: usize) -> io::Result<Self> {
        Ok(Self {
            pool: LocalPoolHandle::new(size),
        })
    }

    pub fn spawn_pinned<F, Fut>(&self, create_task: F)
    where
        F: FnOnce() -> Fut,
        F: Send + 'static,
        Fut: Future<Output = ()> + 'static,
    {
        self.pool.spawn_pinned(create_task);
    }
}

#[derive(Debug)]
pub(crate) struct LocalRuntime {
    local_set: LocalSet,
    rt: TokioRuntime,
}

impl LocalRuntime {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            local_set: LocalSet::new(),
            rt: TokioRuntimeBuilder::new_current_thread()
                .enable_all()
                .build()?,
        })
    }

    pub fn block_on<F>(&self, f: F) -> F::Output
    where
        F: Future + 'static,
        F::Output: 'static,
    {
        self.local_set.block_on(&self.rt, f)
    }
}
