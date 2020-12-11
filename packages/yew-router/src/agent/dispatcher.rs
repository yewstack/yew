//! Dispatcher to RouteAgent.
use crate::{agent::RouteAgent, RouteState};
use std::{
    fmt::{Debug, Error as FmtError, Formatter},
    ops::{Deref, DerefMut},
};
use yew::agent::{Dispatched, Dispatcher};

/// A wrapped dispatcher to the route agent.
///
/// A component that owns and instance of this can send messages to the RouteAgent, but not receive them.
pub struct RouteAgentDispatcher<STATE = ()>(Dispatcher<RouteAgent<STATE>>)
where
    STATE: RouteState;

impl<STATE> RouteAgentDispatcher<STATE>
where
    STATE: RouteState,
{
    /// Creates a new bridge.
    pub fn new() -> Self {
        let dispatcher = RouteAgent::dispatcher();
        RouteAgentDispatcher(dispatcher)
    }
}

impl<STATE> Default for RouteAgentDispatcher<STATE>
where
    STATE: RouteState,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<STATE: RouteState> Debug for RouteAgentDispatcher<STATE> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.debug_tuple("RouteAgentDispatcher").finish()
    }
}

impl<STATE: RouteState> Deref for RouteAgentDispatcher<STATE> {
    type Target = Dispatcher<RouteAgent<STATE>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T: RouteState> DerefMut for RouteAgentDispatcher<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
