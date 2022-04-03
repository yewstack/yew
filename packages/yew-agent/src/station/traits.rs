use pin_project::pin_project;
use std::collections::HashMap;
use std::pin::Pin;

use futures::channel::mpsc;
use futures::future::LocalBoxFuture;
use futures::stream::{FusedStream, Stream, StreamExt};
use futures::task::{Context, Poll};
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;

use crate::worker::{HandlerId, Worker, WorkerScope};

type IoPair<R> = (
    mpsc::UnboundedSender<<R as StationReceivable>::Output>,
    mpsc::UnboundedReceiver<<R as StationReceivable>::Input>,
);

/// A receiver for stations.
#[pin_project]
#[derive(Debug)]
pub struct StationReceiver<I, O>
where
    I: Serialize + for<'de> Deserialize<'de>,
    O: Serialize + for<'de> Deserialize<'de>,
{
    #[pin]
    rx: mpsc::UnboundedReceiver<IoPair<Self>>,
}

impl<I, O> Stream for StationReceiver<I, O>
where
    I: Serialize + for<'de> Deserialize<'de>,
    O: Serialize + for<'de> Deserialize<'de>,
{
    type Item = IoPair<Self>;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();
        this.rx.poll_next(cx)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.rx.size_hint()
    }
}

impl<I, O> FusedStream for StationReceiver<I, O>
where
    I: Serialize + for<'de> Deserialize<'de>,
    O: Serialize + for<'de> Deserialize<'de>,
{
    fn is_terminated(&self) -> bool {
        self.rx.is_terminated()
    }
}

/// A trait to extract input and output type from StationReceiver.
pub trait StationReceivable {
    /// The input message type.
    type Input: Serialize + for<'de> Deserialize<'de>;
    /// The output message type.
    type Output: Serialize + for<'de> Deserialize<'de>;

    /// Creates a StationReceiver.
    fn new(rx: mpsc::UnboundedReceiver<IoPair<Self>>) -> Self;
}

impl<I, O> StationReceivable for StationReceiver<I, O>
where
    I: Serialize + for<'de> Deserialize<'de>,
    O: Serialize + for<'de> Deserialize<'de>,
{
    type Input = I;
    type Output = O;

    fn new(rx: mpsc::UnboundedReceiver<IoPair<Self>>) -> Self {
        Self { rx }
    }
}

/// A station agent.
pub trait Station {
    /// The receiver type.
    type Receiver: StationReceivable;

    /// Start a station.
    fn start(recv: Self::Receiver) -> LocalBoxFuture<'static, ()>;
}

pub(crate) enum StationWorkerMsg {
    StationExited,
}

pub(crate) struct StationWorker<S>
where
    S: 'static + Station,
{
    link: WorkerScope<Self>,
    senders: HashMap<HandlerId, mpsc::UnboundedSender<<Self as Worker>::Input>>,
    tx: mpsc::UnboundedSender<IoPair<S::Receiver>>,
}

impl<S> Worker for StationWorker<S>
where
    S: 'static + Station,
{
    type Input = <S::Receiver as StationReceivable>::Input;
    type Output = <S::Receiver as StationReceivable>::Output;
    type Message = StationWorkerMsg;

    fn create(link: WorkerScope<Self>) -> Self {
        let (tx, rx) = mpsc::unbounded();

        {
            link.send_future(async move {
                let receiver = S::Receiver::new(rx);

                S::start(receiver).await;

                StationWorkerMsg::StationExited
            });
        }

        Self {
            link,
            senders: HashMap::new(),
            tx,
        }
    }

    fn connected(&mut self, id: HandlerId) {
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
                    link.respond(id, m);
                }
            });

            tx
        };

        self.tx
            .unbounded_send((sender, receiver))
            .expect("attempting to connect after destory!");
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Self::Message::StationExited => {
                self.link.close();
            }
        }
    }

    fn received(&mut self, input: Self::Input, id: HandlerId) {
        if let Some(m) = self.senders.get_mut(&id) {
            let _result = m.unbounded_send(input);
        }
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.senders.remove(&id);
    }

    fn destroy(&mut self) -> bool {
        self.tx.close_channel();

        false
    }
}
