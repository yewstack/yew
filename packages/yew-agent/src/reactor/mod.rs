//! This module contains the reactor agent implementation.
//!
//! Reactor agents are agents that receive multiple inputs and send multiple outputs over a single
//! bridge. A reactor is defined as an async function that takes a [ReactorScope]
//! as the argument.
//!
//! The reactor scope is a stream that produces inputs from the bridge and a
//! sink that implements an additional send method to send outputs to the connected bridge.
//! When the bridge disconnects, the output stream and input sink will be closed.
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
//! use futures::sink::SinkExt;
//! use futures::stream::StreamExt;
//! use yew_agent::reactor::{reactor, ReactorScope};
//! #[reactor(MyReactor)]
//! pub async fn my_reactor(mut scope: ReactorScope<ReactorInput, ReactorOutput>) {
//!     while let Some(input) = scope.next().await {
//!         // handles each input.
//!         // ...
//! #       let output = ReactorOutput { /* ... */ };
//!
//!         // sends output
//!         if scope.send(output).await.is_err() {
//!             // sender closed, the bridge is disconnected
//!             break;
//!         }
//!     }
//! }
//! ```

mod bridge;
mod hooks;
mod messages;
mod provider;
mod registrar;
mod scope;
mod spawner;
mod traits;
mod worker;

pub use bridge::{ReactorBridge, ReactorBridgeSinkError};
pub use hooks::{
    use_reactor_bridge, use_reactor_subscription, ReactorEvent, UseReactorBridgeHandle,
    UseReactorSubscriptionHandle,
};
pub use provider::ReactorProvider;
pub(crate) use provider::ReactorProviderState;
pub use registrar::ReactorRegistrar;
pub use scope::{ReactorScope, ReactorScoped};
pub use spawner::ReactorSpawner;
pub use traits::Reactor;
/// A procedural macro to create reactor agents.
pub use yew_agent_macro::reactor;
