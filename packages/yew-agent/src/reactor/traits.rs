use std::collections::HashMap;
use std::fmt;

use futures::channel::mpsc;
use futures::future::LocalBoxFuture;
use futures::stream::StreamExt;
use wasm_bindgen_futures::spawn_local;

use super::messages::{BridgeInput, BridgeOutput};
use super::tx_rx::{ReactorReceivable, ReactorSendable};
use crate::worker::{
    Bincode, Codec, HandlerId, Registrable, Worker, WorkerDestroyHandle, WorkerRegistrar,
    WorkerScope,
};

/// A reactor agent.
pub trait Reactor {
    /// The Reactor Receiver.
    type Receiver: ReactorReceivable;
    /// The Reactor Sender.
    type Sender: ReactorSendable;

    /// Runs a reactor agent.
    fn run(tx: Self::Sender, rx: Self::Receiver) -> LocalBoxFuture<'static, ()>;

    /// Creates a registrar for the current reactor agent.
    fn registrar() -> ReactorRegistrar<Self>
    where
        Self: Sized,
    {
        ReactorRegistrar {
            inner: ReactorWorker::<Self>::registrar(),
        }
    }
}

/// A registrar for reactor agents.
pub struct ReactorRegistrar<R, CODEC = Bincode>
where
    R: Reactor + 'static,
    CODEC: Codec + 'static,
{
    inner: WorkerRegistrar<ReactorWorker<R>, CODEC>,
}

impl<R, CODEC> ReactorRegistrar<R, CODEC>
where
    R: Reactor + 'static,
    CODEC: Codec + 'static,
{
    /// Sets the encoding.
    pub fn encoding<C>(&self) -> ReactorRegistrar<R, C>
    where
        C: Codec + 'static,
    {
        ReactorRegistrar {
            inner: self.inner.encoding::<C>(),
        }
    }

    /// Registers the agent.
    pub fn register(&self) {
        self.inner.register()
    }
}

impl<R, CODEC> fmt::Debug for ReactorRegistrar<R, CODEC>
where
    R: Reactor + 'static,
    CODEC: Codec + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ReactorRegistrar<_>").finish()
    }
}

pub(crate) enum ReactorWorkerMsg {
    ReactorExited(HandlerId),
}

pub(crate) struct ReactorWorker<R>
where
    R: 'static + Reactor,
{
    senders: HashMap<HandlerId, mpsc::UnboundedSender<<R::Receiver as ReactorReceivable>::Input>>,
    destruct_handle: Option<WorkerDestroyHandle<Self>>,
}

impl<R> Worker for ReactorWorker<R>
where
    R: 'static + Reactor,
{
    type Input = BridgeInput<<R::Receiver as ReactorReceivable>::Input>;
    type Message = ReactorWorkerMsg;
    type Output = BridgeOutput<<R::Sender as ReactorSendable>::Output>;

    fn create(_scope: &WorkerScope<Self>) -> Self {
        Self {
            senders: HashMap::new(),
            destruct_handle: None,
        }
    }

    fn update(&mut self, _scope: &WorkerScope<Self>, msg: Self::Message) {
        match msg {
            ReactorWorkerMsg::ReactorExited(id) => {
                self.senders.remove(&id);
            }
        }

        // All reactors have closed themselves, the worker can now close.
        if self.destruct_handle.is_some() && self.senders.is_empty() {
            self.destruct_handle = None;
        }
    }

    fn received(&mut self, scope: &WorkerScope<Self>, input: Self::Input, id: HandlerId) {
        match input {
            // We don't expose any bridge unless they send start message.
            Self::Input::Start => {
                let receiver = {
                    let (tx, rx) = mpsc::unbounded();
                    self.senders.insert(id, tx);
                    R::Receiver::new(rx)
                };

                let sender = {
                    let (tx, mut rx) = mpsc::unbounded();
                    let scope = scope.clone();

                    spawn_local(async move {
                        while let Some(m) = rx.next().await {
                            scope.respond(id, BridgeOutput::Output(m));
                        }

                        scope.respond(id, BridgeOutput::Finish);
                    });

                    R::Sender::new(tx)
                };

                scope.send_future(async move {
                    R::run(sender, receiver).await;

                    ReactorWorkerMsg::ReactorExited(id)
                });
            }

            Self::Input::Input(input) => {
                if let Some(m) = self.senders.get_mut(&id) {
                    let _result = m.unbounded_send(input);
                }
            }
        }
    }

    fn disconnected(&mut self, _scope: &WorkerScope<Self>, id: HandlerId) {
        // We close this channel, but drop it when the reactor has exited itself.
        if let Some(m) = self.senders.get_mut(&id) {
            m.close_channel();
        }
    }

    fn destroy(&mut self, _scope: &WorkerScope<Self>, destruct: WorkerDestroyHandle<Self>) {
        if !self.senders.is_empty() {
            self.destruct_handle = Some(destruct);
        }
    }
}
