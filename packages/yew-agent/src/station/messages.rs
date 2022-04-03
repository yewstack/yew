use serde::{Deserialize, Serialize};

/// The Bridge Input.
#[derive(Serialize, Deserialize)]
pub(crate) enum BridgeInput<I>
where
    I: 'static,
{
    /// Starts the bridge.
    Start,
    /// An input message.
    Input(I),
}

/// The Bridge Output.
#[derive(Debug, Serialize, Deserialize)]
pub enum BridgeOutput<O>
where
    O: 'static,
{
    /// An output message has been received.
    Output(O),
    /// Station has its output sender for current bridge.
    Finish,
}
