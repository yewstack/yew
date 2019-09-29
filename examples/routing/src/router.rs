//! Agent that exposes a usable routing interface to components.

use crate::routing::RouteService;
use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt::Debug;
use stdweb::unstable::TryFrom;
use stdweb::JsSerialize;
use stdweb::Value;
use yew::worker::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Route<T> {
    pub path_segments: Vec<String>,
    pub query: Option<String>,
    pub fragment: Option<String>,
    pub state: T,
}

impl<T> Route<T>
where
    T: JsSerialize + Clone + TryFrom<Value> + Default + 'static,
{
    pub fn to_route_string(&self) -> String {
        let path = self.path_segments.join("/");
        let mut path = format!("/{}", path); // add the leading '/'
        if let Some(ref query) = self.query {
            path = format!("{}?{}", path, query);
        }
        if let Some(ref fragment) = self.fragment {
            path = format!("{}#{}", path, fragment)
        }
        path
    }

    pub fn current_route(route_service: &RouteService<T>) -> Self {
        let path = route_service.get_path(); // guaranteed to always start with a '/'
        let mut path_segments: Vec<String> = path.split("/").map(String::from).collect();
        path_segments.remove(0); // remove empty string that is split from the first '/'

        let mut query: String = route_service.get_query(); // The first character will be a '?'
        let query: Option<String> = if query.len() > 1 {
            query.remove(0);
            Some(query)
        } else {
            None
        };

        let mut fragment: String = route_service.get_fragment(); // The first character will be a '#'
        let fragment: Option<String> = if fragment.len() > 1 {
            fragment.remove(0);
            Some(fragment)
        } else {
            None
        };

        Route {
            path_segments,
            query,
            fragment,
            state: T::default(),
        }
    }
}

pub enum Msg<T>
where
    T: JsSerialize + Clone + Debug + TryFrom<Value> + 'static,
{
    BrowserNavigationRouteChanged((String, T)),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Request<T> {
    /// Changes the route using a RouteInfo struct and alerts connected components to the route change.
    ChangeRoute(Route<T>),
    /// Changes the route using a RouteInfo struct, but does not alert connected components to the route change.
    ChangeRouteNoBroadcast(Route<T>),
    GetCurrentRoute,
}

/// The Router worker holds on to the RouteService singleton and mediates access to it.
pub struct Router<T>
where
    for<'de> T: JsSerialize
        + Clone
        + Debug
        + TryFrom<Value>
        + Default
        + Serialize
        + Deserialize<'de>
        + 'static,
{
    link: AgentLink<Router<T>>,
    route_service: RouteService<T>,
    /// A list of all entities connected to the router.
    /// When a route changes, either initiated by the browser or by the app,
    /// the route change will be broadcast to all listening entities.
    subscribers: HashSet<HandlerId>,
}

impl<T> Agent for Router<T>
where
    for<'de> T: JsSerialize
        + Clone
        + Debug
        + TryFrom<Value>
        + Default
        + Serialize
        + Deserialize<'de>
        + 'static,
{
    type Reach = Context;
    type Message = Msg<T>;
    type Input = Request<T>;
    type Output = Route<T>;

    fn create(link: AgentLink<Self>) -> Self {
        let callback = link.send_back(|route_changed: (String, T)| {
            Msg::BrowserNavigationRouteChanged(route_changed)
        });
        let mut route_service = RouteService::new();
        route_service.register_callback(callback);

        Router {
            link,
            route_service,
            subscribers: HashSet::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::BrowserNavigationRouteChanged((_route_string, state)) => {
                info!("Browser navigated");
                let mut route = Route::current_route(&self.route_service);
                route.state = state;
                for sub in self.subscribers.iter() {
                    self.link.response(*sub, route.clone());
                }
            }
        }
    }

    fn handle(&mut self, msg: Self::Input, who: HandlerId) {
        info!("Request: {:?}", msg);
        match msg {
            Request::ChangeRoute(route) => {
                let route_string: String = route.to_route_string();
                // set the route
                self.route_service.set_route(&route_string, route.state);
                // get the new route. This will contain a default state object
                let route = Route::current_route(&self.route_service);
                // broadcast it to all listening components
                for sub in self.subscribers.iter() {
                    self.link.response(*sub, route.clone());
                }
            }
            Request::ChangeRouteNoBroadcast(route) => {
                let route_string: String = route.to_route_string();
                self.route_service.set_route(&route_string, route.state);
            }
            Request::GetCurrentRoute => {
                let route = Route::current_route(&self.route_service);
                self.link.response(who, route.clone());
            }
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.link
            .response(id, Route::current_route(&self.route_service));
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}
