//! Bridge to RouteAgent.
use crate::agent::RouteAgent;
use crate::{route::Route, RouteState};
use std::{
    fmt::{Debug, Error as FmtError, Formatter},
    ops::{Deref, DerefMut},
};
use yew::Callback;
use yew_agent::Bridge;
use yew_agent::{Bridged, Context};

/// A wrapped bridge to the route agent.
///
/// A component that owns this can send and receive messages from the agent.
pub struct RouteAgentBridge<STATE = ()>(Box<dyn Bridge<RouteAgent<STATE>>>)
where
    STATE: RouteState;

impl<STATE> RouteAgentBridge<STATE>
where
    STATE: RouteState,
{
    /// Creates a new bridge.
    pub fn new(callback: Callback<Route<STATE>>) -> Self {
        let router_agent = RouteAgent::bridge(callback);
        RouteAgentBridge(router_agent)
    }

    /// Experimental, may be removed
    ///
    /// Directly spawn a new Router
    pub fn spawn(callback: Callback<Route<STATE>>) -> Self {
        use yew_agent::Discoverer;
        let router_agent = Context::spawn_or_join(Some(callback));
        RouteAgentBridge(router_agent)
    }
}

impl<STATE: RouteState> Debug for RouteAgentBridge<STATE> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.debug_tuple("RouteAgentBridge").finish()
    }
}

impl<STATE: RouteState> Deref for RouteAgentBridge<STATE> {
    type Target = Box<dyn Bridge<RouteAgent<STATE>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<STATE: RouteState> DerefMut for RouteAgentBridge<STATE> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
