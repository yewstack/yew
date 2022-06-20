//! This module contains the reactor agent implementation.
//!
//! Reactor agents are agents that receive multiple inputs and send multiple outputs over a single
//! bridge. A reactor is defined as an async function that takes a [ReactorReceiver]
//! and a [ReactorSender] as arguments.
//!
//! The reactor receiver is a stream that produces inputs from the bridge. The reactor sender is a
//! sink that implements an additional send method to send outputs to the connected bridge.
//! When the bridge disconnects, the output sender and input receiver will be closed.
//!
//! # Example
//!
//! ```
//! # use serde::{Serialize, Deserialize};
//! # #[derive(Serialize, Deserialize)]
//! # pub struct ReactorInput {}
//! # #[derive(Serialize, Deserialize)]
//! # pub struct ReactorOutput {}
//! #
//! use yew_agent::reactor;
//! use yew_agent::reactor::{ReactorReceiver, ReactorSender};
//! #[reactor(MyReactor)]
//! pub async fn my_reactor(rx: ReactorReceiver<ReactorInput>, tx: ReactorSender<ReactorOutput>) {
//!     while let Some(input) = rx.next().await {
//!         // handles each input.
//!         // ...
//! #       let output = ReactorOutput;
//!
//!         // sends output
//!         if tx.send(output).is_err() {
//!             // sender closed, the bridge is disconnected
//!             break;
//!         }
//!     }
//! }
//! ```

mod hooks;
mod messages;
mod provider;
mod traits;
mod tx_rx;

pub use hooks::{
    use_reactor_bridge, use_reactor_subscription, UseReactorBridgeHandle,
    UseReactorSubscriptionHandle,
};
pub(crate) use messages::ReactorInput;
pub use messages::ReactorOutput;
pub use provider::ReactorProvider;
pub(crate) use traits::ReactorWorker;
pub use traits::{Reactor, ReactorRegistrar};
pub use tx_rx::{ReactorReceivable, ReactorReceiver, ReactorSendable, ReactorSender};
