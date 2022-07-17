use std::future::Future;
use std::sync::Arc;
use std::{fmt, io, thread};

use once_cell::sync::Lazy;
use tokio::runtime::{Builder as TokioRuntimeBuilder, Runtime as TokioRuntime};
use tokio::task::LocalSet;

pub(crate) mod time;

static DEFAULT_WORKER_NAME: &str = "yew-runtime-worker";

// We use a local worker implementation that does not produce a JoinHandle for spawn_pinned.
// This avoids the cost to acquire a JoinHandle.
//
// We will not be able to produce a meaningful JoinHandle until WebAssembly targets support
// unwinding.
//
// See: https://github.com/tokio-rs/tokio/issues/4819
mod local_worker {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use futures::channel::mpsc::UnboundedSender;
    use futures::stream::StreamExt;
    use tokio::task::{spawn_local, LocalSet};

    use super::*;

    type SpawnTask = Box<dyn Send + FnOnce()>;

    pub(crate) struct LocalWorker {
        task_count: Arc<AtomicUsize>,
        tx: UnboundedSender<SpawnTask>,
    }

    impl LocalWorker {
        pub fn new() -> io::Result<Self> {
            let (tx, mut rx) = futures::channel::mpsc::unbounded::<SpawnTask>();

            let task_count: Arc<AtomicUsize> = Arc::default();

            let rt = TokioRuntimeBuilder::new_current_thread()
                .enable_all()
                .build()?;

            thread::Builder::new()
                .name(DEFAULT_WORKER_NAME.into())
                .spawn(move || {
                    let local_set = LocalSet::new();

                    local_set.block_on(&rt, async move {
                        while let Some(m) = rx.next().await {
                            m();
                        }
                    });

                    drop(local_set);
                })?;

            Ok(Self { task_count, tx })
        }

        pub fn task_count(&self) -> usize {
            self.task_count.load(Ordering::Acquire)
        }

        pub fn spawn_pinned<F, Fut>(&self, f: F)
        where
            F: 'static + Send + FnOnce() -> Fut,
            Fut: 'static + Future<Output = ()>,
        {
            let guard = LocalJobCountGuard::new(self.task_count.clone());

            // We ignore the result upon a failure, this can never happen unless the runtime is
            // exiting which all instances of Runtime will be dropped at that time and hence cannot
            // spawn pinned tasks.
            let _ = self.tx.unbounded_send(Box::new(move || {
                spawn_local(async move {
                    let _guard = guard;

                    f().await;
                });
            }));
        }
    }

    pub struct LocalJobCountGuard(Arc<AtomicUsize>);

    impl LocalJobCountGuard {
        fn new(inner: Arc<AtomicUsize>) -> Self {
            inner.fetch_add(1, Ordering::AcqRel);
            LocalJobCountGuard(inner)
        }
    }

    impl Drop for LocalJobCountGuard {
        fn drop(&mut self) {
            self.0.fetch_sub(1, Ordering::AcqRel);
        }
    }
}

use local_worker::LocalWorker;

pub(crate) fn get_default_runtime_size() -> usize {
    thread::available_parallelism()
        .map(|m| m.get())
        .unwrap_or(1)
}

#[inline(always)]
pub(super) fn spawn_local<F>(f: F)
where
    F: Future<Output = ()> + 'static,
{
    tokio::task::spawn_local(f);
}

#[derive(Clone)]
pub(crate) struct Runtime {
    workers: Arc<Vec<LocalWorker>>,
}

impl fmt::Debug for Runtime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Runtime")
            .field("workers", &"Vec<LocalWorker>")
            .finish()
    }
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
        assert!(size > 0, "must have more than 1 worker.");

        let mut workers = Vec::with_capacity(size);

        for _ in 0..size {
            let worker = LocalWorker::new()?;
            workers.push(worker);
        }

        Ok(Self {
            workers: workers.into(),
        })
    }

    fn find_least_busy_local_worker(&self) -> &LocalWorker {
        let mut workers = self.workers.iter();

        let mut worker = workers.next().expect("must have more than 1 worker.");
        let mut task_count = worker.task_count();

        for current_worker in workers {
            if task_count == 0 {
                // We don't have to search until the end.
                break;
            }

            let current_worker_task_count = current_worker.task_count();

            if current_worker_task_count < task_count {
                task_count = current_worker_task_count;
                worker = current_worker;
            }
        }

        worker
    }

    pub fn spawn_pinned<F, Fut>(&self, create_task: F)
    where
        F: FnOnce() -> Fut,
        F: Send + 'static,
        Fut: Future<Output = ()> + 'static,
    {
        let worker = self.find_least_busy_local_worker();
        worker.spawn_pinned(create_task);
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
