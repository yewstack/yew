//! Routing agent.
//!
//! It wraps a route service and allows calls to be sent to it to update every subscriber,
//! or just the element that made the request.
use crate::service::RouteService;

use yew::prelude::worker::*;

use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Error as FmtError, Formatter};

use crate::route::{Route, RouteState};
use log::trace;

mod bridge;
pub use bridge::RouteAgentBridge;

mod dispatcher;
pub use dispatcher::RouteAgentDispatcher;

/// Internal Message used for the RouteAgent.
#[derive(Debug)]
pub enum Msg<STATE> {
    /// Message for when the route is changed.
    BrowserNavigationRouteChanged(Route<STATE>), // TODO make this a route?
}

/// Input message type for interacting with the `RouteAgent'.
#[derive(Serialize, Deserialize, Debug)]
pub enum RouteRequest<T = ()> {
    /// Replaces the most recent Route with a new one and alerts connected components to the route
    /// change.
    ReplaceRoute(Route<T>),
    /// Replaces the most recent Route with a new one, but does not alert connected components to
    /// the route change.
    ReplaceRouteNoBroadcast(Route<T>),
    /// Changes the route using a Route struct and alerts connected components to the route change.
    ChangeRoute(Route<T>),
    /// Changes the route using a Route struct, but does not alert connected components to the
    /// route change.
    ChangeRouteNoBroadcast(Route<T>),
    /// Gets the current route.
    GetCurrentRoute,
}

/// The RouteAgent holds on to the RouteService singleton and mediates access to it.
///
/// It serves as a means to propagate messages to components interested in the state of the current
/// route.
///
/// # Warning
/// All routing-related components/agents/services should use the same type parameter across your application.
///
/// If you use multiple agents with different types, then the Agents won't be able to communicate to
/// each other and associated components may not work as intended.
pub struct RouteAgent<STATE = ()>
where
    STATE: RouteState,
{
    // In order to have the AgentLink<Self> below, apparently T must be constrained like this.
    // Unfortunately, this means that everything related to an agent requires this constraint.
    link: AgentLink<RouteAgent<STATE>>,
    /// The service through which communication with the browser happens.
    route_service: RouteService<STATE>,
    /// A list of all entities connected to the router.
    /// When a route changes, either initiated by the browser or by the app,
    /// the route change will be broadcast to all listening entities.
    subscribers: HashSet<HandlerId>,
}

impl<STATE: RouteState> Debug for RouteAgent<STATE> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.debug_struct("RouteAgent")
            .field("link", &"-")
            .field("route_service", &self.route_service)
            .field("subscribers", &self.subscribers.len())
            .finish()
    }
}

impl<STATE> Agent for RouteAgent<STATE>
where
    STATE: RouteState,
{
    type Input = RouteRequest<STATE>;
    type Message = Msg<STATE>;
    type Output = Route<STATE>;
    type Reach = Context<Self>;

    fn create(link: AgentLink<RouteAgent<STATE>>) -> Self {
        let callback = link.callback(Msg::BrowserNavigationRouteChanged);
        let mut route_service = RouteService::new();
        route_service.register_callback(callback);

        RouteAgent {
            link,
            route_service,
            subscribers: HashSet::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::BrowserNavigationRouteChanged(route) => {
                trace!("Browser navigated");
                for sub in &self.subscribers {
                    self.link.respond(*sub, route.clone());
                }
            }
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn handle_input(&mut self, msg: Self::Input, who: HandlerId) {
        match msg {
            RouteRequest::ReplaceRoute(route) => {
                let route_string: String = route.to_string();
                self.route_service.replace_route(&route_string, route.state);
                let route = self.route_service.get_route();
                for sub in &self.subscribers {
                    self.link.respond(*sub, route.clone());
                }
            }
            RouteRequest::ReplaceRouteNoBroadcast(route) => {
                let route_string: String = route.to_string();
                self.route_service.replace_route(&route_string, route.state);
            }
            RouteRequest::ChangeRoute(route) => {
                let route_string: String = route.to_string();
                // set the route
                self.route_service.set_route(&route_string, route.state);
                // get the new route.
                let route = self.route_service.get_route();
                // broadcast it to all listening components
                for sub in &self.subscribers {
                    self.link.respond(*sub, route.clone());
                }
            }
            RouteRequest::ChangeRouteNoBroadcast(route) => {
                let route_string: String = route.to_string();
                self.route_service.set_route(&route_string, route.state);
            }
            RouteRequest::GetCurrentRoute => {
                let route = self.route_service.get_route();
                self.link.respond(who, route);
            }
        }
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}
