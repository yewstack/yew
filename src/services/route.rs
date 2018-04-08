//! This module contains the implementation of a service for
//! setting the url and responding to changes to the url
//! that are initiated by the browser..

use stdweb::web::{History, Location, window};
use stdweb::Value;

use stdweb::web::{EventListenerHandle, IEventTarget};
use stdweb::web::event::PopStateEvent;
use stdweb::unstable::TryFrom;
use html::Callback;

use url::{Url};
use std::ops::Add;


/// An alias for `Result<RouteInfo, RoutingError>`.
pub type RouteResult = Result<RouteInfo, RoutingError>;

/// Service used for routing
pub struct RouteService {
    history: History,
    location: Location,
    event_listener: Option<EventListenerHandle>,
    callback: Option<Callback<RouteResult>>
}

/// A subset of the url crate's Url object that can be passed
/// to crate consumers to deal with routing.
#[derive(Clone, PartialEq, Debug)]
pub struct RouteInfo {
    /// The segments of the path string
    pub path_segments: Vec<String>,
    /// The query parameter
    pub query: Option<String>, // TODO it might make sense to store the query as a hashmap
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
    CouldNotParseRoute {
        /// In the event that url crate can't parse the route string, the route string will be passed back to the crate user to use.
        route: String
    },
    /// If the full Url can't be parsed this will be returned
    CouldNotParseUrl {
        /// This will contain the full url, not just the route.
        full_url: String
    },
    /// An error indicating that the string passed to the `RouteInfo::parse()` function did not start with a slash.
    RouteDoesNotStartWithSlash,
    /// An error indicating that the string passed to the `RouteInfo::parse()` function did not contain ary characters
    RouteIsEmpty,
    /// Indicates that the url could not be retrieved from the Location API.
    CouldNotGetLocationHref
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
            .map_err(|_| RoutingError::CouldNotParseRoute { route: route_string.to_string() })
    }

    /// Converts the RouteInfo into a string that can be matched upon,
    /// as well as stored in the History Api.
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

    /// Gets the path segment at the specified index.
    pub fn get_segment_at_index<'a>(&'a self, index: usize) -> Option<&'a str> {
        self.path_segments.get(index).map(String::as_str)
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
            location: window().location().expect("Could not find Location API, routing is not supported in your browser."),
            event_listener: None,
            callback: None
        }
    }



    /// Will return the current route info based on the location API.
    // TODO this should probably return a RouteResult and avoid expecting
    pub fn get_current_route_info(&mut self) -> RouteInfo {
        // If the location api errors, recover by redirecting to a valid address
        let href = self.get_location().expect("Couldn't get href from location Api");
        let url = Url::parse(&href).expect("The href returned from the location api should always be parsable.");
        RouteInfo::from(url)
    }

    /// Registers the router.
    /// There can only be one router.
    /// The component in which it is set up will be the source from which routing logic emanates.
    pub fn register_router(&mut self, callback: Callback<RouteResult>)
    {
        if let Some(_) = self.event_listener {
            panic!("You cannot register two separate routers.");
        }

        // Hold on to the callback so it can be used to update the main router component
        // when a user clicks a link, independent of the event listener.
        self.callback = Some(callback.clone());

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
    /// If the route is not already set to the string corresponding to the provided RouteInfo,
    /// the history will be updated, and the routing callback will be invoked.
    pub fn set_route<T: Into<RouteInfo>>(&mut self, route_info: T) {
        let route_info: RouteInfo = route_info.into();
        if route_info != self.get_current_route_info() {
            let route_string: String = route_info.to_string();
            println!("Setting route: {}", route_string); // this line needs to be removed eventually
            let r = js! {
                return @{route_string.clone()}
            };
            // Set the state using the history API
            self.history.push_state(r, "", Some(&route_string));
            self.go_to_current_route();
        }
    }


    /// Replaces the url with the one provided by the route info.
    /// This will not invoke the routing callback.
    pub fn replace_url<T: Into<RouteInfo>>(&mut self, route_info: T) {
        let route_string: String = route_info.into().to_string();
        let r = js! {
            return @{route_string.clone()}
        };
        let _ = self.history.replace_state(r, "", Some(&route_string));
    }

    /// Based on the location API, set the route by calling the callback.
    pub fn go_to_current_route(&mut self) {
        if let Some(ref cb) = self.callback {

            let route_result: RouteResult = match self.get_location() {
                Ok(full_url) => {
                     Url::parse(&full_url)
                        .map(RouteInfo::from)
                        .map_err(|_|RoutingError::CouldNotParseUrl {full_url: full_url.to_string()})
                }
                Err(e) => Err(e)
            };
            cb.emit(route_result)

        } else {
            eprintln!("Callback was never set.")
        }
    }

    /// Gets the location.
    pub fn get_location(&self) -> Result<String, RoutingError> {
        self.location.href().map_err(|_|RoutingError::CouldNotGetLocationHref)
    }
}
