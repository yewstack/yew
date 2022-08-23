//! A multi-producer, single-receiver channel.

#[doc(inline)]
pub use tokio::sync::mpsc::*;
#[doc(inline)]
pub use tokio_stream::wrappers::{ReceiverStream, UnboundedReceiverStream};
