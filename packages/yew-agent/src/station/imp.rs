use std::collections::HashMap;

use futures::channel::mpsc;
use futures::stream::StreamExt;
use wasm_bindgen_futures::spawn_local;

use super::messages::{BridgeInput, BridgeOutput};
use super::recv::{IoPair, StationReceivable};
use super::Station;
use crate::reactor::{ReactorReceivable, ReactorReceiver, ReactorSendable, ReactorSender};
use crate::worker::{HandlerId, Worker, WorkerScope};

pub(crate) enum StationWorkerMsg {
    StationExited,
}

pub(crate) struct StationWorker<S>
where
    S: 'static + Station,
{
    senders: HashMap<HandlerId, mpsc::UnboundedSender<<S::Receiver as StationReceivable>::Input>>,
    tx: mpsc::UnboundedSender<IoPair<S::Receiver>>,
    closing: bool,
}

impl<S> Worker for StationWorker<S>
where
    S: 'static + Station,
{
    type Input = BridgeInput<<S::Receiver as StationReceivable>::Input>;
    type Message = StationWorkerMsg;
    type Output = BridgeOutput<<S::Receiver as StationReceivable>::Output>;

    fn create(scope: &WorkerScope<Self>) -> Self {
        let (tx, rx) = mpsc::unbounded();

        {
            scope.send_future(async move {
                let receiver = S::Receiver::new(rx);

                S::run(receiver).await;

                StationWorkerMsg::StationExited
            });
        }

        Self {
            senders: HashMap::new(),
            tx,
            closing: false,
        }
    }

    fn update(&mut self, scope: &WorkerScope<Self>, msg: Self::Message) {
        match msg {
            Self::Message::StationExited => {
                assert!(self.closing, "station agent closed before it should do so");
                scope.close();
            }
        }
    }

    fn received(&mut self, scope: &WorkerScope<Self>, input: Self::Input, id: HandlerId) {
        match input {
            // We don't expose any bridge unless they send start message.
            Self::Input::Start => {
                let receiver = {
                    let (tx, rx) = mpsc::unbounded();
                    self.senders.insert(id, tx);
                    rx
                };

                let sender = {
                    let (tx, mut rx) = mpsc::unbounded();
                    let link = scope.clone();

                    spawn_local(async move {
                        while let Some(m) = rx.next().await {
                            link.respond(id, BridgeOutput::Output(m));
                        }
                        link.respond(id, BridgeOutput::Finish);
                    });

                    tx
                };

                self.tx
                    .unbounded_send((ReactorSender::new(sender), ReactorReceiver::new(receiver)))
                    .expect("attempting to connect after destory!");
            }

            Self::Input::Input(input) => {
                if let Some(m) = self.senders.get_mut(&id) {
                    let _result = m.unbounded_send(input);
                }
            }
        }
    }

    fn disconnected(&mut self, _scope: &WorkerScope<Self>, id: HandlerId) {
        self.senders.remove(&id);
    }

    fn destroy(&mut self, _scope: &WorkerScope<Self>) -> bool {
        self.tx.close_channel();
        self.closing = true;

        false
    }
}
