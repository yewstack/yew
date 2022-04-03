//! This module contains the implementation of the station agent.
//!
//! Station is a kind of agent that can receive many inputs and send many outputs
//! over multiple bridges. A station is defined as an async function that takes a [StationReceiver]
//! as the only argument.
//!
//! The station receiver is a stream that produces output sender - input receiver pairs. Each pair
//! will be connected to a bridge. When the bridge disconnects, the output receiver will be closed.
//!
//! When a station receives a destroy message, the station receiver will be closed.
//!
//! # Example
//!
//! ```
//! # use serde::{Serialize, Deserialize};
//! # #[derive(Serialize, Deserialize)]
//! # pub struct StationInput {}
//! # #[derive(Serialize, Deserialize)]
//! # pub struct StationOutput {}
//! #
//! use yew_agent::station;
//! use yew_agent::station::StationReceiver;
//! #[station(MyStation)]
//! pub async fn my_station(recv: StationReceiver<StationInput, StationOutput>) {
//!     while let Some((tx, rx)) = recv.next().await {
//!         // handles each bridge.
//!     }
//! }
//! ```
//!
//! # Panics
//!
//! The underlying worker will panic if the station receiver is dropped before all bridges are
//! disconnected or the station coroutine stops running (returns).
//! To avoid this behaviour, the main station function should run until the station receiver
//! reaches the end.

use futures::future::LocalBoxFuture;

mod hooks;
mod imp;
mod messages;
mod provider;
mod recv;

pub use hooks::{
    use_station_bridge, use_station_subscription, UseStationBridgeHandle,
    UseStationSubscriptionHandle,
};
pub use messages::BridgeOutput;
pub use provider::StationProvider;
pub use recv::{StationReceivable, StationReceiver};

/// A station agent.
pub trait Station {
    /// The receiver type.
    type Receiver: StationReceivable;

    /// Runs a station.
    fn run(recv: Self::Receiver) -> LocalBoxFuture<'static, ()>;
}
