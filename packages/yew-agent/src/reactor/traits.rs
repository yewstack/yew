use futures::stream::StreamExt;

use super::tx_rx::{ReactorReceivable, ReactorSendable};
use crate::station;
use crate::station::StationReceiver;

/// A reactor agent.
pub trait Reactor {
    /// The Reactor Receiver.
    type Receiver: ReactorReceivable;
    /// The Reactor Sender.
    type Sender: ReactorSendable;

    /// Start a reactor agent.
    fn start(tx: Self::Sender, rx: Self::Receiver);
}

#[station(ReactorStation)]
pub(crate) async fn reactor_station<R>(
    mut rx: StationReceiver<
        <R::Receiver as ReactorReceivable>::Input,
        <R::Sender as ReactorSendable>::Output,
    >,
) where
    R: 'static + Reactor,
{
    while let Some((tx, rx)) = rx.next().await {
        let (tx, rx) = (R::Sender::new(tx), R::Receiver::new(rx));
        R::start(tx, rx);
    }
}
