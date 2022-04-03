use futures::channel::oneshot;
use futures::future::join_all;
use futures::future::LocalBoxFuture;
use futures::stream::StreamExt;
use wasm_bindgen_futures::spawn_local;

use super::tx_rx::{ReactorReceivable, ReactorSendable};
use crate::station;
use crate::station::StationReceiver;

/// A reactor agent.
pub trait Reactor {
    /// The Reactor Receiver.
    type Receiver: ReactorReceivable;
    /// The Reactor Sender.
    type Sender: ReactorSendable;

    /// Runs a reactor agent.
    fn run(tx: Self::Sender, rx: Self::Receiver) -> LocalBoxFuture<'static, ()>;
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
    let mut futures = Vec::new();

    while let Some((tx, rx)) = rx.next().await {
        let (tx, rx) = (R::Sender::new(tx), R::Receiver::new(rx));
        let (on_finish, notify_finished) = oneshot::channel();

        spawn_local(async move {
            R::run(tx, rx).await;
            let _result = on_finish.send(());
        });

        futures.push(async move {
            let _result = notify_finished.await;
        });
    }

    // We need to wait until all reactors exit.
    join_all(futures).await;
}
