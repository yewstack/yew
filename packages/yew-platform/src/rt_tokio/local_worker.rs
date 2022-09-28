//! We use a local worker implementation that does not produce a JoinHandle for spawn_pinned.
//! This avoids the cost to acquire a JoinHandle.
//!
//! See: [tokio-rs/tokio#4819](https://github.com/tokio-rs/tokio/issues/4819)
//!
//! We will not be able to produce a meaningful JoinHandle until WebAssembly targets support
//! unwinding.

use std::cell::RefCell;
use std::future::Future;
use std::marker::PhantomData;
use std::sync::Arc;
use std::{io, thread};

static DEFAULT_WORKER_NAME: &str = "yew-runtime-worker";

use std::sync::atomic::{AtomicUsize, Ordering};

use futures::channel::mpsc::UnboundedSender;
use futures::stream::StreamExt;
use tokio::task::{spawn_local, LocalSet};

type SpawnTask = Box<dyn Send + FnOnce()>;

thread_local! {
    static TASK_COUNT: RefCell<Option<Arc<AtomicUsize>>> = RefCell::new(None);
    static LOCAL_SET: LocalSet = LocalSet::new();
}

pub(crate) struct LocalWorker {
    task_count: Arc<AtomicUsize>,
    tx: UnboundedSender<SpawnTask>,
}

impl LocalWorker {
    pub fn new() -> io::Result<Self> {
        let (tx, mut rx) = futures::channel::mpsc::unbounded::<SpawnTask>();

        let task_count: Arc<AtomicUsize> = Arc::default();

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;

        {
            let task_count = task_count.clone();
            thread::Builder::new()
                .name(DEFAULT_WORKER_NAME.into())
                .spawn(move || {
                    TASK_COUNT.with(move |m| {
                        *m.borrow_mut() = Some(task_count);
                    });

                    LOCAL_SET.with(|local_set| {
                        local_set.block_on(&rt, async move {
                            while let Some(m) = rx.next().await {
                                m();
                            }
                        });
                    });
                })?;
        }

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

#[derive(Debug, Clone)]
pub(crate) struct LocalHandle {
    // This type is not send or sync.
    _marker: PhantomData<*const ()>,
    task_count: Arc<AtomicUsize>,
}

impl LocalHandle {
    pub fn try_current() -> Option<Self> {
        // We cache the handle to prevent borrowing RefCell.
        thread_local! {
            static LOCAL_HANDLE: Option<LocalHandle> = TASK_COUNT
            .with(|m| m.borrow().clone())
            .map(|task_count| LocalHandle { task_count, _marker: PhantomData });
        }

        LOCAL_HANDLE.with(|m| m.clone())
    }

    pub fn current() -> Self {
        Self::try_current().expect("outside of Yew runtime.")
    }

    pub fn spawn_local<F>(&self, f: F)
    where
        F: Future<Output = ()> + 'static,
    {
        let guard = LocalJobCountGuard::new(self.task_count.clone());

        LOCAL_SET.with(move |local_set| {
            local_set.spawn_local(async move {
                let _guard = guard;

                f.await;
            })
        });
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use futures::channel::oneshot;
    use tokio::test;
    use tokio::time::timeout;
    use yew::platform::Runtime;

    use super::*;

    #[test]
    async fn test_local_handle_exists() {
        assert!(LocalHandle::try_current().is_none());

        let runtime = Runtime::default();
        let (tx, rx) = oneshot::channel();

        runtime.spawn_pinned(move || async move {
            tx.send(LocalHandle::try_current().is_some())
                .expect("failed to send");
        });

        timeout(Duration::from_secs(5), rx)
            .await
            .expect("task timed out")
            .expect("failed to receive");
    }

    #[test]
    async fn test_local_handle_spawns_on_same_worker() {
        assert!(LocalHandle::try_current().is_none());

        let runtime = Runtime::default();
        let (tx1, rx1) = oneshot::channel();
        let (tx2, rx2) = oneshot::channel();

        runtime.spawn_pinned(move || async move {
            let handle = LocalHandle::current();

            tx1.send(std::thread::current().id())
                .expect("failed to send");

            handle.spawn_local(async move {
                tx2.send(std::thread::current().id())
                    .expect("failed to send");
            })
        });

        let result1 = timeout(Duration::from_secs(5), rx1)
            .await
            .expect("task timed out")
            .expect("failed to receive");
        let result2 = timeout(Duration::from_secs(5), rx2)
            .await
            .expect("task timed out")
            .expect("failed to receive");

        assert_eq!(result1, result2);
    }
}
