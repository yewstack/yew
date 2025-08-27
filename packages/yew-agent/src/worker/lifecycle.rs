use wasm_bindgen::prelude::*;

use super::messages::ToWorker;
use super::native_worker::{DedicatedWorker, WorkerSelf};
use super::scope::{WorkerDestroyHandle, WorkerScope};
use super::traits::Worker;
use super::Shared;

pub(crate) struct WorkerState<W>
where
    W: Worker,
{
    worker: Option<(W, WorkerScope<W>)>,
    to_destroy: bool,
}

impl<W> WorkerState<W>
where
    W: Worker,
{
    pub fn new() -> Self {
        WorkerState {
            worker: None,
            to_destroy: false,
        }
    }
}

/// Internal Worker lifecycle events
pub(crate) enum WorkerLifecycleEvent<W: Worker> {
    /// Request to create the scope
    Create(WorkerScope<W>),

    /// Internal Worker message
    Message(W::Message),

    /// External Messages from bridges
    Remote(ToWorker<W>),

    /// Destroy the Worker
    Destroy,
}

pub(crate) struct WorkerRunnable<W: Worker> {
    pub state: Shared<WorkerState<W>>,
    pub event: WorkerLifecycleEvent<W>,
}

impl<W> WorkerRunnable<W>
where
    W: Worker + 'static,
{
    pub fn run(self) {
        let mut state = self.state.borrow_mut();

        // We should block all event other than message after a worker is destroyed.
        match self.event {
            WorkerLifecycleEvent::Create(scope) => {
                if state.to_destroy {
                    return;
                }
                state.worker = Some((W::create(&scope), scope));
            }
            WorkerLifecycleEvent::Message(msg) => {
                if let Some((worker, scope)) = state.worker.as_mut() {
                    worker.update(scope, msg);
                }
            }
            WorkerLifecycleEvent::Remote(ToWorker::Connected(id)) => {
                if state.to_destroy {
                    return;
                }

                let (worker, scope) = state
                    .worker
                    .as_mut()
                    .expect_throw("worker was not created to process connected messages");

                worker.connected(scope, id);
            }
            WorkerLifecycleEvent::Remote(ToWorker::ProcessInput(id, inp)) => {
                if state.to_destroy {
                    return;
                }

                let (worker, scope) = state
                    .worker
                    .as_mut()
                    .expect_throw("worker was not created to process inputs");

                worker.received(scope, inp, id);
            }
            WorkerLifecycleEvent::Remote(ToWorker::Disconnected(id)) => {
                if state.to_destroy {
                    return;
                }

                let (worker, scope) = state
                    .worker
                    .as_mut()
                    .expect_throw("worker was not created to process disconnected messages");

                worker.disconnected(scope, id);
            }
            WorkerLifecycleEvent::Remote(ToWorker::Destroy) => {
                if state.to_destroy {
                    return;
                }

                state.to_destroy = true;

                let (worker, scope) = state
                    .worker
                    .as_mut()
                    .expect_throw("trying to destroy not existent worker");

                let destruct = WorkerDestroyHandle::new(scope.clone());

                worker.destroy(scope, destruct);
            }

            WorkerLifecycleEvent::Destroy => {
                state
                    .worker
                    .take()
                    .expect_throw("worker is not initialised or already destroyed");

                DedicatedWorker::worker_self().close();
            }
        }
    }
}
