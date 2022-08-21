use std::future::Future;
use std::sync::Arc;
use std::{fmt, io};

use once_cell::sync::Lazy;

pub(crate) mod time;

mod local_worker;

pub(crate) use local_worker::LocalHandle;
use local_worker::LocalWorker;

pub(crate) fn get_default_runtime_size() -> usize {
    // We use num_cpus as std::thread::available_parallelism() does not take
    // system resource constraint (e.g.: cgroups) into consideration.
    num_cpus::get()
}

#[inline(always)]
pub(super) fn spawn_local<F>(f: F)
where
    F: Future<Output = ()> + 'static,
{
    match LocalHandle::try_current() {
        Some(m) => {
            // If within a Yew runtime, use a local handle increases the local task count.
            m.spawn_local(f);
        }
        None => {
            tokio::task::spawn_local(f);
        }
    }
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
