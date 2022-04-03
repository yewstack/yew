use std::pin::Pin;

use futures::channel::mpsc;
use futures::stream::{FusedStream, Stream};
use futures::task::{Context, Poll};
use pin_project::pin_project;
use serde::{Deserialize, Serialize};

pub(crate) type IoPair<R> = (
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
