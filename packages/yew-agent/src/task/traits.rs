use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use crate::worker::{HandlerId, Worker, WorkerScope};

/// A task agent.
///
/// For this kind of agent, each input will receive 1 response.
pub trait Task: Sized {
    /// The Input Message.
    type Input: Serialize + for<'de> Deserialize<'de>;
    /// The Output Message.
    type Output: Serialize + for<'de> Deserialize<'de>;

    /// Runs a task.
    fn run(input: Self::Input, respond: Box<dyn FnOnce(Self::Output)>);
}

#[derive(Debug)]
pub(crate) struct TaskWorker<T>
where
    T: 'static + Task,
{
    _marker: PhantomData<T>,
}

impl<T> Worker for TaskWorker<T>
where
    T: 'static + Task,
{
    type Input = (usize, T::Input);
    type Message = ();
    type Output = (usize, T::Output);

    fn create(_scope: &WorkerScope<Self>) -> Self {
        Self {
            _marker: PhantomData,
        }
    }

    fn update(&mut self, _scope: &WorkerScope<Self>, _msg: Self::Message) {}

    fn received(&mut self, scope: &WorkerScope<Self>, input: Self::Input, handler_id: HandlerId) {
        let task_id = input.0;

        let respond = {
            let scope = scope.clone();

            Box::new(move |output| {
                scope.respond(handler_id, (task_id, output));
            })
        };

        T::run(input.1, respond);
    }
}
