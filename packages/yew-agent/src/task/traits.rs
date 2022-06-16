use std::collections::HashSet;
use std::fmt;
use std::marker::PhantomData;

use futures::future::LocalBoxFuture;
use serde::{Deserialize, Serialize};

use crate::worker::{
    Bincode, Codec, HandlerId, Registrable, Worker, WorkerDestroyHandle, WorkerRegistrar,
    WorkerScope,
};

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

    /// Creates a registrar for the current task agent.
    fn registrar() -> TaskRegistrar<Self>
    where
        Self: Sized,
    {
        TaskRegistrar {
            inner: TaskWorker::<Self>::registrar(),
        }
    }
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

#[derive(PartialEq, Eq, Hash)]
pub(crate) struct TaskId {
    handler_id: HandlerId,
    raw_task_id: usize,
}

pub(crate) enum TaskWorkerMsg<T>
where
    T: Task,
{
    TaskFinished { task_id: TaskId, output: T::Output },
}

pub(crate) struct TaskWorker<T>
where
    T: 'static + Task,
{
    _marker: PhantomData<T>,
    task_ids: HashSet<TaskId>,
    destruct_handle: Option<WorkerDestroyHandle<Self>>,
}

impl<T> Worker for TaskWorker<T>
where
    T: 'static + Task,
{
    type Input = (usize, T::Input);
    type Message = TaskWorkerMsg<T>;
    type Output = (usize, T::Output);

    fn create(_scope: &WorkerScope<Self>) -> Self {
        Self {
            _marker: PhantomData,
            task_ids: Default::default(),
            destruct_handle: None,
        }
    }

    fn update(&mut self, scope: &WorkerScope<Self>, msg: Self::Message) {
        let TaskWorkerMsg::TaskFinished { task_id, output } = msg;

        self.task_ids.remove(&task_id);

        let TaskId {
            raw_task_id,
            handler_id,
        } = task_id;

        scope.respond(handler_id, (raw_task_id, output));

        if self.task_ids.is_empty() {
            self.destruct_handle = None;
        }
    }

    fn received(&mut self, scope: &WorkerScope<Self>, input: Self::Input, handler_id: HandlerId) {
        let (raw_task_id, input) = input;

        let task_id = TaskId {
            handler_id,
            raw_task_id,
        };

        scope.send_future(async move {
            let output = T::run(input).await;

            TaskWorkerMsg::TaskFinished { task_id, output }
        });
    }

    fn destroy(&mut self, _scope: &WorkerScope<Self>, destruct: WorkerDestroyHandle<Self>) {
        if !self.task_ids.is_empty() {
            self.destruct_handle = Some(destruct);
        }
    }
}
