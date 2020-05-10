//! This module contains types to support multi-threading in Yew.

mod link;
mod local;
mod pool;
mod worker;

pub use link::AgentLink;
pub(crate) use link::*;
pub use local::{Context, Job};
pub(crate) use pool::*;
pub use pool::{Dispatched, Dispatcher};
pub use worker::{Private, Public, Threaded};

use crate::callback::Callback;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::{Deref, DerefMut};

/// Declares the behavior of the agent.
pub trait Agent: Sized + 'static {
    /// Reach capability of the agent.
    type Reach: Discoverer;
    /// Type of an input message.
    type Message;
    /// Incoming message type.
    type Input: Serialize + for<'de> Deserialize<'de>;
    /// Outgoing message type.
    type Output: Serialize + for<'de> Deserialize<'de>;

    /// Creates an instance of an agent.
    fn create(link: AgentLink<Self>) -> Self;

    /// This method called on every update message.
    fn update(&mut self, msg: Self::Message);

    /// This method called on when a new bridge created.
    fn connected(&mut self, _id: HandlerId) {}

    /// This method called on every incoming message.
    fn handle_input(&mut self, msg: Self::Input, id: HandlerId);

    /// This method called on when a new bridge destroyed.
    fn disconnected(&mut self, _id: HandlerId) {}

    /// This method called when the agent is destroyed.
    fn destroy(&mut self) {}

    /// Represents the name of loading resorce for remote workers which
    /// have to live in a separate files.
    fn name_of_resource() -> &'static str {
        "main.js"
    }

    /// Signifies if resource is a module.
    /// This has pending browser support.
    fn is_module() -> bool {
        false
    }
}

/// Id of responses handler.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Copy)]
pub struct HandlerId(usize, bool);

impl HandlerId {
    fn new(id: usize, respondable: bool) -> Self {
        HandlerId(id, respondable)
    }
    fn raw_id(self) -> usize {
        self.0
    }
    /// Indicates if a handler id corresponds to callback in the Agent runtime.
    pub fn is_respondable(self) -> bool {
        self.1
    }
}

/// This trait allows registering or getting the address of a worker.
pub trait Bridged: Agent + Sized + 'static {
    /// Creates a messaging bridge between a worker and the component.
    fn bridge(callback: Callback<Self::Output>) -> Box<dyn Bridge<Self>>;
}

impl<T> Bridged for T
where
    T: Agent,
{
    fn bridge(callback: Callback<Self::Output>) -> Box<dyn Bridge<Self>> {
        Self::Reach::spawn_or_join(Some(callback))
    }
}

/// Determine a visibility of an agent.
#[doc(hidden)]
pub trait Discoverer {
    /// Spawns an agent and returns `Bridge` implementation.
    fn spawn_or_join<AGN: Agent>(_callback: Option<Callback<AGN::Output>>) -> Box<dyn Bridge<AGN>>;
}

/// Bridge to a specific kind of worker.
pub trait Bridge<AGN: Agent> {
    /// Send a message to an agent.
    fn send(&mut self, msg: AGN::Input);
}
