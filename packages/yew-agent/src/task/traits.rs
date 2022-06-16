use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;

use futures::future::LocalBoxFuture;
use serde::{Deserialize, Serialize};

use crate::worker::{HandlerId, Worker, WorkerDestroyHandle, WorkerScope};

/// A task agent.
///
/// For this kind of agent, each input will receive 1 response.
pub trait Task: Sized {
    /// The Input Message.
    type Input: Serialize + for<'de> Deserialize<'de>;
    /// The Output Message.
    type Output: Serialize + for<'de> Deserialize<'de>;

    /// Runs a task.
    fn run(input: Self::Input) -> LocalBoxFuture<'static, Self::Output>;
}

pub(crate) enum TaskWorkerMsg<T>
where
    T: Task,
{
    TaskFinished {
        handler_id: HandlerId,
        output: T::Output,
        task_id: usize,
    },
}

pub(crate) struct TaskWorker<T>
where
    T: 'static + Task,
{
    _marker: PhantomData<T>,
    task_ids: HashMap<HandlerId, HashSet<usize>>,
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
        let TaskWorkerMsg::TaskFinished {
            handler_id,
            task_id,
            output,
        } = msg;

        let handler_empty = if let Some(m) = self.task_ids.get_mut(&handler_id) {
            m.remove(&task_id);

            m.is_empty()
        } else {
            false
        };

        scope.respond(handler_id, (task_id, output));

        if handler_empty {
            self.task_ids.remove(&handler_id);
        }

        if self.task_ids.is_empty() {
            self.destruct_handle = None;
        }
    }

    fn received(&mut self, scope: &WorkerScope<Self>, input: Self::Input, handler_id: HandlerId) {
        let (task_id, input) = input;

        match self.task_ids.entry(handler_id) {
            Entry::Occupied(mut m) => {
                m.get_mut().insert(task_id);
            }
            Entry::Vacant(m) => {
                let mut set = HashSet::new();
                set.insert(task_id);
                m.insert(set);
            }
        }

        scope.send_future(async move {
            let output = T::run(input).await;

            TaskWorkerMsg::TaskFinished {
                handler_id,
                task_id,
                output,
            }
        });
    }

    fn destroy(&mut self, _scope: &WorkerScope<Self>, destruct: WorkerDestroyHandle<Self>) {
        if !self.task_ids.is_empty() {
            self.destruct_handle = Some(destruct);
        }
    }
}
