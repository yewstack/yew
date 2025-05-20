use serde::{Deserialize, Serialize};

use super::handler_id::HandlerId;
use super::traits::Worker;

/// Serializable messages to worker
#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum ToWorker<W>
where
    W: Worker,
{
    /// Client is connected
    Connected(HandlerId),
    /// Incoming message to Worker
    ProcessInput(HandlerId, W::Input),
    /// Client is disconnected
    Disconnected(HandlerId),
    /// Worker should be terminated
    Destroy,
}

/// Serializable messages sent by worker to consumer
#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum FromWorker<W>
where
    W: Worker,
{
    /// Worker sends this message when `wasm` bundle has loaded.
    WorkerLoaded,
    /// Outgoing message to consumer
    ProcessOutput(HandlerId, W::Output),
}
