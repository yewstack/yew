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
//! use yew_agent::reactor::{reactor, ReactorReceiver, ReactorSender};
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
mod provider;

#[doc(inline)]
pub use gloo_worker::reactor::{
    Reactor, ReactorBridge, ReactorRegistrar, ReactorScope, ReactorScoped, ReactorSpawner,
};
pub use hooks::{
    use_reactor_bridge, use_reactor_subscription, ReactorEvent, UseReactorBridgeHandle,
    UseReactorSubscriptionHandle,
};
pub use provider::ReactorProvider;
pub(crate) use provider::ReactorProviderState;
/// A procedural macro to create reactor agents.
pub use yew_agent_macro::reactor;
