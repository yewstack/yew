use yew::services::routing::RouteService;

use yew::prelude::worker::*;

use std::collections::HashSet;

use stdweb::Value;
use stdweb::JsSerialize;
use stdweb::unstable::TryFrom;


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize) ]
pub struct RouteInfo {
    pub path_segments: Vec<String>,
    pub query: Option<String>,
    pub fragment: Option<String>
}

impl RouteInfo {
    pub fn to_string(&self) -> String {
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

    pub fn current_route<T>(route_service: &RouteService<T>) -> Self
        where T: JsSerialize + Clone + TryFrom<Value> + 'static
    {
        let path = route_service.get_path();
        let path_segments: Vec<String> = path.split("/").map(String::from).collect();
        let query = Some(route_service.get_query()); // TODO this is incorrect, I should properly check the string to see if there is any information.
        let fragment = Some(route_service.get_fragment());

        RouteInfo {
            path_segments,
            query,
            fragment
        }
    }
}

pub enum Msg {
    BrowserNavigationRouteChanged((String, ())),
}


#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    RouteChanged(RouteInfo)
}
impl Transferable for Response {}

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    /// Changes the route using a RouteInfo struct.
    ChangeRouteInfo(RouteInfo),
    /// Changes the route using a String.
    /// Its use is discouraged.
    ChangeRoute(String)
}

impl Transferable for Request {}

/// The Router worker holds on to the RouteService singleton and mediates access to it.
pub struct Router {
    link: AgentLink<Router>,
    route_service: RouteService<()>,
    /// A list of all entities connected to the router.
    /// When a route changes, either initiated by the browser or by the app,
    /// the route change will be broadcast to all listening entities.
    subscribers: HashSet<HandlerId>
}

impl Agent for Router {
    type Reach = Context;
    type Message = Msg;
    type Input = Request;
    type Output = Response;

    fn create(link: AgentLink<Self>) -> Self {
        let callback = link.send_back(|route_changed: (String, ())| Msg::BrowserNavigationRouteChanged(route_changed));
        let mut route_service = RouteService::new();
        route_service.register_callback(callback);
        Router {
            link,
            route_service,
            subscribers: HashSet::new()
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::BrowserNavigationRouteChanged((_route, _state)) => {
                info!("Browser navigated");
                let route = RouteInfo::current_route(&self.route_service);
                for sub in self.subscribers.iter() {
                    self.link.response(*sub, Response::RouteChanged(route.clone()));
                }
            }
        }
    }

    fn handle(&mut self, msg: Self::Input, _who: HandlerId) {
        info!("Request: {:?}", msg);
        match msg {
            Request::ChangeRoute(route) => {
                // set the route
                self.route_service.set_route(&route, ());
                // get the new route
                let route = RouteInfo::current_route(&self.route_service);
                // broadcast it to all listening components
                for sub in self.subscribers.iter() {
                    self.link.response(*sub, Response::RouteChanged(route.clone()));
                }
            }
            Request::ChangeRouteInfo(route) => {
                let route_string = route.to_string();
                // set the route
                self.route_service.set_route(&route_string, ());
                // get the new route
                let route = RouteInfo::current_route(&self.route_service);
                // broadcast it to all listening components
                for sub in self.subscribers.iter() {
                    self.link.response(*sub, Response::RouteChanged(route.clone()));
                }
            }
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }
    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}
