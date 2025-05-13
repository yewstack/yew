use serde::{Deserialize, Serialize};

/// The Bridge Input.
#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum ReactorInput<I> {
    /// An input message.
    Input(I),
}

/// The Bridge Output.
#[derive(Debug, Serialize, Deserialize)]
pub enum ReactorOutput<O> {
    /// An output message has been received.
    Output(O),
    /// Reactor for current bridge has exited.
    Finish,
}
