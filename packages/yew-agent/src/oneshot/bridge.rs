use futures::stream::StreamExt;
use pinned::mpsc;
use pinned::mpsc::UnboundedReceiver;

use super::traits::Oneshot;
use super::worker::OneshotWorker;
use crate::codec::Codec;
use crate::worker::{WorkerBridge, WorkerSpawner};

/// A connection manager for components interaction with oneshot workers.
#[derive(Debug)]
pub struct OneshotBridge<N>
where
    N: Oneshot + 'static,
{
    inner: WorkerBridge<OneshotWorker<N>>,
    rx: UnboundedReceiver<N::Output>,
}

impl<N> OneshotBridge<N>
where
    N: Oneshot + 'static,
{
    #[inline(always)]
    pub(crate) fn new(
        inner: WorkerBridge<OneshotWorker<N>>,
        rx: UnboundedReceiver<N::Output>,
    ) -> Self {
        Self { inner, rx }
    }

    #[inline(always)]
    pub(crate) fn register_callback<CODEC>(
        spawner: &mut WorkerSpawner<OneshotWorker<N>, CODEC>,
    ) -> UnboundedReceiver<N::Output>
    where
        CODEC: Codec,
    {
        let (tx, rx) = mpsc::unbounded();
        spawner.callback(move |output| {
            let _ = tx.send_now(output);
        });

        rx
    }

    /// Forks the bridge.
    ///
    /// This method creates a new bridge that can be used to execute tasks on the same worker
    /// instance.
    pub fn fork(&self) -> Self {
        let (tx, rx) = mpsc::unbounded();
        let inner = self.inner.fork(Some(move |output| {
            let _ = tx.send_now(output);
        }));

        Self { inner, rx }
    }

    /// Run the current oneshot worker once in the current worker instance.
    pub async fn run(&mut self, input: N::Input) -> N::Output {
        // &mut self guarantees that the bridge will be
        // exclusively borrowed during the time the oneshot worker is running.
        self.inner.send(input);

        // For each bridge, there can only be 1 active task running on the worker instance.
        // The next output will be the output for the input that we just sent.
        self.rx
            .next()
            .await
            .expect("failed to receive result from worker")
    }
}
