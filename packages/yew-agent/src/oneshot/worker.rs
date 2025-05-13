use super::traits::Oneshot;
use crate::worker::{HandlerId, Worker, WorkerDestroyHandle, WorkerScope};

pub(crate) enum Message<T>
where
    T: Oneshot,
{
    Finished {
        handler_id: HandlerId,
        output: T::Output,
    },
}

pub(crate) struct OneshotWorker<T>
where
    T: 'static + Oneshot,
{
    running_tasks: usize,
    destruct_handle: Option<WorkerDestroyHandle<Self>>,
}

impl<T> Worker for OneshotWorker<T>
where
    T: 'static + Oneshot,
{
    type Input = T::Input;
    type Message = Message<T>;
    type Output = T::Output;

    fn create(_scope: &WorkerScope<Self>) -> Self {
        Self {
            running_tasks: 0,
            destruct_handle: None,
        }
    }

    fn update(&mut self, scope: &WorkerScope<Self>, msg: Self::Message) {
        let Message::Finished { handler_id, output } = msg;

        self.running_tasks -= 1;

        scope.respond(handler_id, output);

        if self.running_tasks == 0 {
            self.destruct_handle = None;
        }
    }

    fn received(&mut self, scope: &WorkerScope<Self>, input: Self::Input, handler_id: HandlerId) {
        self.running_tasks += 1;

        scope.send_future(async move {
            let output = T::create(input).await;

            Message::Finished { handler_id, output }
        });
    }

    fn destroy(&mut self, _scope: &WorkerScope<Self>, destruct: WorkerDestroyHandle<Self>) {
        if self.running_tasks > 0 {
            self.destruct_handle = Some(destruct);
        }
    }
}
