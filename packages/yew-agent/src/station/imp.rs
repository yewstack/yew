use std::collections::HashMap;

use futures::channel::mpsc;
use futures::stream::StreamExt;
use wasm_bindgen_futures::spawn_local;

use super::messages::{BridgeInput, BridgeOutput};
use super::recv::{IoPair, StationReceivable};
use super::Station;
use crate::worker::{HandlerId, Worker, WorkerScope};

pub(crate) enum StationWorkerMsg {
    StationExited,
}

pub(crate) struct StationWorker<S>
where
    S: 'static + Station,
{
    link: WorkerScope<Self>,
    senders: HashMap<HandlerId, mpsc::UnboundedSender<<S::Receiver as StationReceivable>::Input>>,
    tx: mpsc::UnboundedSender<IoPair<S::Receiver>>,
    closing: bool,
}

impl<S> Worker for StationWorker<S>
where
    S: 'static + Station,
{
    type Input = BridgeInput<<S::Receiver as StationReceivable>::Input>;
    type Output = BridgeOutput<<S::Receiver as StationReceivable>::Output>;
    type Message = StationWorkerMsg;

    fn create(link: WorkerScope<Self>) -> Self {
        let (tx, rx) = mpsc::unbounded();

        {
            link.send_future(async move {
                let receiver = S::Receiver::new(rx);

                S::run(receiver).await;

                StationWorkerMsg::StationExited
            });
        }

        Self {
            link,
            senders: HashMap::new(),
            tx,
            closing: false,
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Self::Message::StationExited => {
                assert!(self.closing, "station agent closed before it should do so");
                self.link.close();
            }
        }
    }

    fn received(&mut self, input: Self::Input, id: HandlerId) {
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
                    let link = self.link.clone();

                    spawn_local(async move {
                        while let Some(m) = rx.next().await {
                            link.respond(id, BridgeOutput::Output(m));
                        }
                        link.respond(id, BridgeOutput::Finish);
                    });

                    tx
                };

                self.tx
                    .unbounded_send((sender, receiver))
                    .expect("attempting to connect after destory!");
            }

            Self::Input::Input(input) => {
                if let Some(m) = self.senders.get_mut(&id) {
                    let _result = m.unbounded_send(input);
                }
            }
        }
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.senders.remove(&id);
    }

    fn destroy(&mut self) -> bool {
        self.tx.close_channel();
        self.closing = true;

        false
    }
}
