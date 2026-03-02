use std::collections::HashMap;
use std::convert::Infallible;

use futures::sink;
use futures::stream::StreamExt;
use pinned::mpsc;
use pinned::mpsc::UnboundedSender;
use wasm_bindgen_futures::spawn_local;

use super::messages::{ReactorInput, ReactorOutput};
use super::scope::ReactorScoped;
use super::traits::Reactor;
use crate::worker::{HandlerId, Worker, WorkerDestroyHandle, WorkerScope};

pub(crate) enum Message {
    ReactorExited(HandlerId),
}

pub(crate) struct ReactorWorker<R>
where
    R: 'static + Reactor,
{
    senders: HashMap<HandlerId, UnboundedSender<<R::Scope as ReactorScoped>::Input>>,
    destruct_handle: Option<WorkerDestroyHandle<Self>>,
}

impl<R> Worker for ReactorWorker<R>
where
    R: 'static + Reactor,
{
    type Input = ReactorInput<<R::Scope as ReactorScoped>::Input>;
    type Message = Message;
    type Output = ReactorOutput<<R::Scope as ReactorScoped>::Output>;

    fn create(_scope: &WorkerScope<Self>) -> Self {
        Self {
            senders: HashMap::new(),
            destruct_handle: None,
        }
    }

    fn update(&mut self, scope: &WorkerScope<Self>, msg: Self::Message) {
        match msg {
            Self::Message::ReactorExited(id) => {
                scope.respond(id, ReactorOutput::Finish);
                self.senders.remove(&id);
            }
        }

        // All reactors have closed themselves, the worker can now close.
        if self.destruct_handle.is_some() && self.senders.is_empty() {
            self.destruct_handle = None;
        }
    }

    fn connected(&mut self, scope: &WorkerScope<Self>, id: HandlerId) {
        let from_bridge = {
            let (tx, rx) = mpsc::unbounded();
            self.senders.insert(id, tx);

            rx
        };

        let to_bridge = {
            let scope_ = scope.clone();
            let (tx, mut rx) = mpsc::unbounded();
            spawn_local(async move {
                while let Some(m) = rx.next().await {
                    scope_.respond(id, ReactorOutput::Output(m));
                }
            });

            sink::unfold((), move |_, item: <R::Scope as ReactorScoped>::Output| {
                let tx = tx.clone();

                async move {
                    let _ = tx.send_now(item);

                    Ok::<(), Infallible>(())
                }
            })
        };

        let reactor_scope = ReactorScoped::new(from_bridge, to_bridge);

        let reactor = R::create(reactor_scope);

        scope.send_future(async move {
            reactor.await;

            Message::ReactorExited(id)
        });
    }

    fn received(&mut self, _scope: &WorkerScope<Self>, input: Self::Input, id: HandlerId) {
        match input {
            Self::Input::Input(input) => {
                if let Some(m) = self.senders.get_mut(&id) {
                    let _result = m.send_now(input);
                }
            }
        }
    }

    fn disconnected(&mut self, _scope: &WorkerScope<Self>, id: HandlerId) {
        // We close this channel, but drop it when the reactor has exited itself.
        if let Some(m) = self.senders.get_mut(&id) {
            m.close_now();
        }
    }

    fn destroy(&mut self, _scope: &WorkerScope<Self>, destruct: WorkerDestroyHandle<Self>) {
        if !self.senders.is_empty() {
            self.destruct_handle = Some(destruct);
        }
    }
}
