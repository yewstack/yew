//! This module contains the implementation of a service for
//! setting the url and responding to changes to the url
//! that are initiated by the browser..

use stdweb::web::History;
use stdweb::web::Location;
use stdweb::web::window;
//use stdweb::web::Window;
//use stdweb;
use stdweb::Value;
use stdweb::web::EventListenerHandle;
use stdweb::web::event::PopStateEvent;
use stdweb::web::IEventTarget;
use stdweb::unstable::TryFrom;
use html::Callback;

use html::Env;
use html::Component;
use url_lib::{Url};
use std::ops::Add;


type RouteDidChange = bool;

/// When the url parser cannot parse a route string, it returns the route string instead.
pub type RawRoute = String;

/// Service used for routing
pub struct RouteService {
    history: History,
    location: Location,
    event_listener: Option<EventListenerHandle>,
    callback: Option<Callback<Result<RouteInfo, RoutingError>>>
}

/// A subset of the url crate's Url object that can be passed
/// to crate consumers to deal with routing.
#[derive(Clone, PartialEq, Debug)]
pub struct RouteInfo {
    /// The segments of the path string
    pub path_segments: Vec<String>,
    /// The query parameter
    pub query: Option<String>,
    /// The fragment
    pub fragment: Option<String>
}

impl Add for RouteInfo {
    type Output = RouteInfo;
    fn add(self, rhs: RouteInfo) -> RouteInfo {
        let mut path_segments = self.path_segments;
        path_segments.extend(rhs.path_segments);
        RouteInfo {
            path_segments,
            query: rhs.query,
            fragment: rhs.fragment
        }
    }
}


/// An error that can occur in the course of routing
#[derive(Debug, Clone, PartialEq)]
pub enum RoutingError {
    /// An error indicating that the string passed to the `RouteInfo::parse()` function couldn't parse the url.
    CouldNotParseUrl{
        /// In the event that url crate can't parse the route string, the route string will be passed back to the crate user to use.
        raw_route: RawRoute
    },
    /// An error indicating that the string passed to the `RouteInfo::parse()` function did not start with a slash.
    RouteDoesNotStartWithSlash,
    /// An error indicating that the string passed to the `RouteInfo::parse()` function did not contain ary characters
    RouteIsEmpty
}


impl RouteInfo {
    /// This expects a string with a leading slash`
    pub fn parse(route_string: &str) -> Result<RouteInfo, RoutingError> {
        // Perform some validation on the string before parsing it.
        if let Some(first_character) = route_string.chars().next() {
            if first_character != '/' {
                eprintln!("does not start with slash: '{}'", route_string);
                return Err(RoutingError::RouteDoesNotStartWithSlash)
            }
        } else {
           return Err(RoutingError::RouteIsEmpty)
        }

        let full_url = format!("http://dummy_url.com{}", route_string);
        Url::parse(&full_url)
            .map(RouteInfo::from)
            .map_err(|_| RoutingError::CouldNotParseUrl{ raw_route: route_string.to_string()})
    }

    #[test]
    fn parse_test() {
        let route_info = RouteInfo {
            path_segments: vet!["/path".to_string()],
            query: None,
            fragment: None
        };
        let parsed_route_info = RouteInfo::parse("/path");
        assert_eq!(route_info, parsed_route_info);
    }
}

impl Into<String> for RouteInfo {
    fn into(self) -> String {

        let path = self.path_segments.join("/");
        let mut path = format!("/{}", path); // add the leading '/'
        if let Some(query) = self.query {
            path = format!("{}?{}", path, query);
        }
        if let Some(fragment) = self.fragment {
            path = format!("{}#{}", path, fragment)
        }
        path
    }
}

impl From<Url> for RouteInfo {
    fn from(url: Url) -> RouteInfo {
        RouteInfo {
            path_segments: url
                .path_segments()
                .expect("The route should always start with '/', so this should never error.")
                .map(str::to_string)
                .collect::<Vec<String>>(),
            query: url.query().map(str::to_string),
            fragment: url.fragment().map(str::to_string)
        }
    }
}


impl RouteService {

    /// Creates a new route service
    pub fn new() -> RouteService {
        RouteService {
            history: window().history(),
            location: window().location().unwrap(),
            event_listener: None,
            callback: None
        }
    }

    /// Creates the callback used in the main routing logic.
    ///
    /// The callback takes a string, parses it into a url, and then uses the result of that
    /// to create a message that the component will use to update itself with.
    pub fn create_routing_callback<C, CTX>(context: &mut Env<CTX, C>) -> Callback<Result<RouteInfo, RoutingError>>
        where
            C: Component<CTX>,
            C::Msg: From<Result<RouteInfo, RoutingError>>,
            CTX: 'static
    {
        return context.send_back(|route_info: Result<RouteInfo, RoutingError>| {
            println!("Callback path changed {:?}", route_info);
            C::Msg::from(route_info)
        })
    }

    /// Will return the current route info based on the location API.
    pub fn get_route_info_from_current_path(&mut self) -> RouteInfo {
        // If the location api errors, recover by redirecting to a valid address
        let href = self.location.href().expect("Couldn't get href from location Api");
        let url = Url::parse(&href).expect("The href returned from the location api should always be parsable.");
        RouteInfo::from(url)
    }

    /// Registers the router.
    /// There can only be one router.
    /// The component in which it is set up will be the source from which routing logic emanates.
    pub fn register_router<T, C, CTX>(&mut self, callback: Callback<Result<RouteInfo, RoutingError>>)
    {
        if let Some(_) = self.event_listener {
            panic!("You cannot register two separate routers.");
        }

        // Hold on to the callback so it can be used to update the main router component
        // when a user clicks a link.
        self.callback = Some(callback.clone());

        let location = &self.location;

        // Set the event listener to listen for the history's pop state events and call the callback when that occurs
        self.event_listener = Some( window().add_event_listener(move |event: PopStateEvent| {
            let state_value: Value = event.state();

            if let Ok(state) = String::try_from(state_value) {
                callback.emit(RouteInfo::parse(&state))
            } else {
                eprintln!("Nothing farther back in history, not calling routing callback.");
            }
        }));
    }


    /// Sets the route via the history api.
    /// This does not by itself make any changes to Yew's state.
    fn set_route(&mut self, route_info: RouteInfo) -> RouteDidChange {
        if route_info != self.get_route_info_from_current_path() {
            let route_string: String = route_info.into();
            println!("Setting route: {}", route_string); // this line needs to be removed eventually
            let r = js! {
                return @{route_string.clone()}
            };
            // Set the state using the history API
            self.history.push_state(r, "", Some(&route_string));
            true
        } else {
            false
        }
    }

    /// Sets the browser's url to the route,
    /// then notifies the main router that the route has changed and it needs to
    /// figure out what to update.
    ///
    /// This second step is necessary because just pushing the state onto the history api won't
    /// cause the callback to be called. The callback needs to be called via go_to_current_route().
    pub fn call_link<T: Into<RouteInfo>>(&mut self, route_info: T) {
        println!("calling link"); // This needs to be removed eventually
        if self.set_route(route_info.into()) {
            self.go_to_current_route();
        }
    }

    /// Based on the location API, set the route by calling the callback.
    pub fn go_to_current_route(&mut self) {
        if let Some(ref cb) = self.callback {
            let full_url: String = self.get_location();
            println!("go_to_current_route: {}", full_url); // This needs to be removed eventually.
            match Url::parse(&full_url) {
                Ok(url) => cb.emit(Ok(url.into())),
                Err(_) => cb.emit(Err(RoutingError::CouldNotParseUrl {raw_route: full_url}))
            }
        } else {
            eprintln!("Callback was never set.")
        }
    }

    /// Gets the location.
    pub fn get_location(&self) -> String {
        self.location.href().expect("Couldn't get location.")
    }
}
