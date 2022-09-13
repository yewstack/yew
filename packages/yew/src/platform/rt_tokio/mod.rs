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

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::time::Duration;

    use futures::channel::oneshot;
    use tokio::sync::Barrier;
    use tokio::test;
    use tokio::time::timeout;

    use super::*;

    #[test]
    async fn test_spawn_pinned_least_busy() {
        let runtime = Runtime::new(2).expect("failed to create runtime.");

        let (tx1, rx1) = oneshot::channel();
        let (tx2, rx2) = oneshot::channel();

        let bar = Arc::new(Barrier::new(2));

        {
            let bar = bar.clone();
            runtime.spawn_pinned(move || async move {
                bar.wait().await;
                tx1.send(std::thread::current().id())
                    .expect("failed to send!");
            });
        }

        runtime.spawn_pinned(move || async move {
            bar.wait().await;
            tx2.send(std::thread::current().id())
                .expect("failed to send!");
        });

        let result1 = timeout(Duration::from_secs(5), rx1)
            .await
            .expect("task timed out")
            .expect("failed to receive");
        let result2 = timeout(Duration::from_secs(5), rx2)
            .await
            .expect("task timed out")
            .expect("failed to receive");

        // first task and second task are not on the same thread.
        assert_ne!(result1, result2);
    }

    #[test]
    async fn test_spawn_local_within_send() {
        let runtime = Runtime::default();

        let (tx, rx) = oneshot::channel();

        runtime.spawn_pinned(move || async move {
            tokio::task::spawn(async move {
                // tokio::task::spawn_local cannot spawn tasks outside of a local context.
                //
                // yew::platform::spawn_local can spawn tasks within a Send task as long as running
                // under a Yew Runtime.
                spawn_local(async move {
                    tx.send(()).expect("failed to send!");
                })
            });
        });

        timeout(Duration::from_secs(5), rx)
            .await
            .expect("task timed out")
            .expect("failed to receive");
    }
}
