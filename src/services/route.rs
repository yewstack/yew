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
use url_lib::{Url, ParseError};


type RouteDidChange = bool;

/// Service used for routing
pub struct RouteService {
    history: History,
    location: Location,
    event_listener: Option<EventListenerHandle>,
    callback: Option<Callback<String>>
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
                .expect("The path segments because the routes should always start with '/'")
                .map(str::to_string)
                .collect::<Vec<String>>(),
            query: url.query().map(str::to_string),
            fragment: url.fragment().map(str::to_string)
        }
    }
}

impl From<Vec<PathSegment>> for RouteInfo {
    fn from(segments: Vec<PathSegment>) -> Self {
        RouteInfo {
            path_segments: segments
                .iter()
                .filter_map(|ps| ps.0.clone())
                .collect::<Vec<String>>(),
           query: None,
           fragment: None
        }
    }
}

/// A wrapper around Option<String> that can be used to create a RouteInfo.
/// It enforces that the contained string does not contain a '/' character.
pub struct PathSegment(Option<String>);


//pub struct ContainsSlash;

// TODO I would like for this to be TryFrom, but that isn't stabilized yet, and yew's implementation isn't public.
// I want this to allow the user to explicitly handle errors,
// like if they are passing in user defined strings into the path.
impl<'a> From<&'a str> for PathSegment {
//    type Error = ContainsSlash;
    fn from(string: &'a str) -> PathSegment {
        if string.contains('/') {
            //Err(ContainsSlash)
            panic!("Used a '/' in the path, that is not allowed.")
        } else {
            if string != "" {
                PathSegment(Some(string.to_string()))
            } else {
                PathSegment(None)
            }
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
    pub fn create_routing_callback<'a, T: 'a, C, CTX>(context: &mut Env<CTX, C>) -> Callback<String>
        where
            T: From<RouteInfo>,
            T: Into<PathSegment>, // This isn't used here, but I would like to enforce that anything that can be routed to needs to be able to convert itself into a path.
            C: Component<CTX>,
            C::Msg: From<T>,
            C::Msg: From<ParseError>,
            CTX: 'static
    {
        return context.send_back(|href: String| {
            println!("Callback path changed {}", href);
            match Url::parse(&href) {
                Ok(url) => {
                    let route_info = RouteInfo::from(url);
                    C::Msg::from(T::from(route_info))
                }
                Err(parse_error) => {
                    C::Msg::from(parse_error)
                }
            }
        })
    }

    /// Will return the current route info based on the location API.
    pub fn get_route_info_from_current_path(&mut self) -> RouteInfo {
        // If the location api errors, recover by redirecting to a valid address
        let href = self.location.href().unwrap_or("http://dummy_url.com/".to_string());
        let url = Url::parse(&href).expect("The href returned from the location api should always be parsable.");
        RouteInfo::from(url)
    }

    /// Registers the router.
    /// There can only be one router.
    /// The component in which it is set up will be the source from which routing logic emanates.
    pub fn register_router<T, C, CTX>(&mut self, callback: Callback<String>)
    {
        if let Some(_) = self.event_listener {
            panic!("You cannot register two separate routers.");
        }

        // Hold on to the callback so it can be used to update the main router component
        // when a user clicks a link.
        self.callback = Some(callback.clone());

        // Set the event listener to listen for the history's pop state events and call the callback when that occurs
        self.event_listener = Some( window().add_event_listener(move |event: PopStateEvent| {
            let state_value: Value = event.state();

            // The route gotten from the popped state isn't enough to parse the url,
            // so we get the href from the location api and use that instead
            if let Ok(_) = String::try_from(state_value) {
                if let Some(location) = window().location() {
                    if let Ok(href) = location.href() {
                        println!("Full url: {}", href);
                        callback.emit(href) // Pass the url string into callback
                    } else {
                        eprint!("Couldn't get the href");
                    }
                } else {
                    eprintln!("Couldn't get the location API.")
                }
            } else {
                eprintln!("Nothing farther back in history, not calling routing callback.");
            }
        }));
    }


    // Currently this gets called twice per link-update. Once when the link is called,
    // and once again when Msg::Notification is fired in the main router and call_link() is called.
    // I don't really like that, but its not the end of the world.
    /// Sets the route via the history api.
    /// This does not by itself make any changes to Yew's state.
    fn set_route<T: Into<RouteInfo>>(&mut self, route: T) -> RouteDidChange {
        let route: RouteInfo = route.into();
        let route: String = route.into();
        // TODO I don't like the need to create a full url in order to parse it using the url crate
        match Url::parse(&format!("http://dummy_url.com{}", route)) {
            Ok(url) => {
                let route_info = RouteInfo::from(url);
                if route_info != self.get_route_info_from_current_path() {
                    println!("Setting route: {}", route); // this line needs to be removed eventually
                    let r = js! {
                        return @{route.clone()}
                    };
                    // Set the state using the history API
                    self.history.push_state(r, "", Some(&route));
                    true
                } else {
        //          eprintln!("Routes are the same. ({}) No update will take place.", url.path());
                    false
                }
            }
            Err(e) => {
                eprintln!("couldn't parse the new route, with error: {}", e);
                false
            }
        }
    }

    /// Sets the browser's url to the route,
    /// then notifies the main router that the route has changed and it needs to
    /// figure out what to update.
    ///
    /// This second step is necessary because just pushing the state onto the history api won't
    /// cause the callback to be called. The callback needs to be called via go_to_current_route().
    pub fn call_link<T: Into<RouteInfo>>(&mut self, route: T) {
        println!("calling link"); // This needs to be removed eventually
        if self.set_route(route) {
            self.go_to_current_route();
        }
    }

    /// Based on the location API, set the route by calling the callback.
    pub fn go_to_current_route(&mut self) {
        if let Some(ref cb) = self.callback {
            let full_url: String = self.get_location();
            println!("go_to_current_route: {}", full_url); // This needs to be removed eventually.
            cb.emit(full_url)
        } else {
            eprintln!("Callback was never set.")
        }
    }

    /// Gets the location.
    pub fn get_location(&self) -> String {
        self.location.href().unwrap()
    }
}
