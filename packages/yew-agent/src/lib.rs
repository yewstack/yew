//! This module contains Yew's web worker implementation.
//!
//! There're 2 kinds of agents:
//!
//! ### Private
//!
//! Each time a bridge is created with the `use_private_bridge`, a new instance
//! of agent is spawned. This allows parallel computing between agents.
//!
//! ### Public
//!
//! Public agents are shared among all children of a [PublicAgentProvider].
//! Only 1 instance will be spawned for each public agents provider.

mod primitives;
mod private;

#[doc(inline)]
pub use primitives::{Agent, AgentScope, Bridge, Spawnable, Spawner};
pub use private::{use_private_bridge, UsePrivateBridgeHandle};
