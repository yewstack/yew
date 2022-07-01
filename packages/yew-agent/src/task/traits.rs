use std::collections::HashSet;
use std::fmt;
use std::marker::PhantomData;

use futures::future::LocalBoxFuture;
use serde::{Deserialize, Serialize};

use crate::worker::{HandlerId, Worker, WorkerDestroyHandle, WorkerRegistrar, WorkerScope};
use crate::{Bincode, Codec, Registrable};

/// A task agent.
///
/// For this kind of agent, each input will receive 1 output.
pub trait Task: Sized {
    /// The Input Message.
    type Input: Serialize + for<'de> Deserialize<'de>;
    /// The Output Message.
    type Output: Serialize + for<'de> Deserialize<'de>;

    /// Runs a task.
    fn run(input: Self::Input) -> LocalBoxFuture<'static, Self::Output>;
}

/// A registrar for task agents.
pub struct TaskRegistrar<T, CODEC = Bincode>
where
    T: Task + 'static,
    CODEC: Codec + 'static,
{
    inner: WorkerRegistrar<TaskWorker<T>, CODEC>,
}

impl<T, CODEC> TaskRegistrar<T, CODEC>
where
    T: Task + 'static,
    CODEC: Codec + 'static,
{
    /// Creates a new Task Registrar.
    pub fn new() -> TaskRegistrar<T> {
        TaskRegistrar {
            inner: TaskWorker::<T>::registrar(),
        }
    }

    /// Sets the encoding.
    pub fn encoding<C>(&self) -> TaskRegistrar<T, C>
    where
        C: Codec + 'static,
    {
        TaskRegistrar {
            inner: self.inner.encoding::<C>(),
        }
    }

    /// Registers the agent.
    pub fn register(&self) {
        self.inner.register()
    }
}

impl<T, CODEC> fmt::Debug for TaskRegistrar<T, CODEC>
where
    T: Task + 'static,
    CODEC: Codec + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TaskRegistrar<_>").finish()
    }
}

pub(crate) enum TaskWorkerMsg<T>
where
    T: Task,
{
    TaskFinished {
        handler_id: HandlerId,
        output: T::Output,
    },
}

pub(crate) struct TaskWorker<T>
where
    T: 'static + Task,
{
    _marker: PhantomData<T>,
    task_ids: HashSet<HandlerId>,
    destruct_handle: Option<WorkerDestroyHandle<Self>>,
}

impl<T> Worker for TaskWorker<T>
where
    T: 'static + Task,
{
    type Input = T::Input;
    type Message = TaskWorkerMsg<T>;
    type Output = T::Output;

    fn create(_scope: &WorkerScope<Self>) -> Self {
        Self {
            _marker: PhantomData,
            task_ids: Default::default(),
            destruct_handle: None,
        }
    }

    fn update(&mut self, scope: &WorkerScope<Self>, msg: Self::Message) {
        let TaskWorkerMsg::TaskFinished { handler_id, output } = msg;

        self.task_ids.remove(&handler_id);

        scope.respond(handler_id, output);

        if self.task_ids.is_empty() {
            self.destruct_handle = None;
        }
    }

    fn received(&mut self, scope: &WorkerScope<Self>, input: Self::Input, handler_id: HandlerId) {
        scope.send_future(async move {
            let output = T::run(input).await;

            TaskWorkerMsg::TaskFinished { handler_id, output }
        });
    }

    fn destroy(&mut self, _scope: &WorkerScope<Self>, destruct: WorkerDestroyHandle<Self>) {
        if !self.task_ids.is_empty() {
            self.destruct_handle = Some(destruct);
        }
    }
}
