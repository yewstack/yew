use std::future::Future;

/// A future-based worker that for each input, one output is produced.
pub trait Oneshot: Future {
    /// Incoming message type.
    type Input;

    /// Creates an oneshot worker.
    fn create(input: Self::Input) -> Self;
}
