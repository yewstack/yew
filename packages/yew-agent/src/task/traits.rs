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
    link: WorkerScope<Self>,
}

impl<T> Worker for TaskWorker<T>
where
    T: 'static + Task,
{
    type Input = (usize, T::Input);
    type Output = (usize, T::Output);
    type Message = ();

    fn create(link: WorkerScope<Self>) -> Self {
        Self {
            _marker: PhantomData,
            link,
        }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn received(&mut self, input: Self::Input, handler_id: HandlerId) {
        let task_id = input.0;

        let respond = {
            let link = self.link.clone();

            Box::new(move |output| {
                link.respond(handler_id, (task_id, output));
            })
        };

        T::run(input.1, respond);
    }
}
