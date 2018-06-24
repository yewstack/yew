//! A number of utilities that make working with routes safer.

use url::Url;
use std::ops::Add;
use std::usize;
use context::route::RoutingError;

//#[derive(PartialEq, Debug, Clone)]
//pub enum Router<T>
//    where T: Routable +
//        Debug +
//        Clone
//{
//    Route(T),
//    Path(Vec<String>)
//}
//
//impl<T: Routable + Debug + Clone> Router<T> {
//    pub fn resolve_route(self) -> T {
//        match self {
//            Router::Route(route) => route,
//            Router::Path(path_components) => T::route(path_components)
//        }
//    }
//}
//
//pub trait Routable {
//    fn route(path_components: Vec<String>) -> Self;
//}




// ------ Copied from prior routing impl ---------



/// Subsection of a RouteInfo produced by iterating over the RouteInfo.
pub enum RouteSection {
    /// When iterating over a RouteInfo, Nodes will be produced when the segment isn't the last in
    /// the vector of path_segments.
    Node {
        /// The path segment.
        segment: String
    },
    /// When iterating over a RouteInfo, Leafs will be produced when the segment is the last in
    /// the vector of path_segments.
    /// This includes the segment, as well as the query and fragment.
    Leaf {
        /// The query string.
        query: Option<String>,
        /// The fragment.
        fragment: Option<String>
    }
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

impl Iterator for RouteInfo {
    type Item = RouteSection;
    fn next(&mut self) -> Option<RouteSection> {

        match self.path_segments.len() {
            1...usize::MAX => {
                let mut first_element = self.path_segments.drain(0..1);
                let node = RouteSection::Node {
                    segment: first_element.next().unwrap()
                };
                Some(node)
            }
            _ => {
                // Return None if no meaningful leaf can be created.
                if let None = self.query {
                    if let None = self.fragment {
                        return None
                    }
                }

                let leaf = RouteSection::Leaf {
                    query: self.query.take(),
                    fragment: self.fragment.take()
                };

                Some(leaf)
            }
        }
    }
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





///// For the route service to choose to render the component, the following needs to be implemented.
//pub trait Routable {
//    /// converts itself to a section
//    fn to_part(&self) -> RouteSection;
//    /// Takes part of a route and converts it to Properties that are used to set itself.
//    fn tune_from_part(route_section: RouteSection) -> Option<Self> where Self: Sized;
//
//}

/// Works with a RouterComponent to set its child
pub trait Router {
    /// Form a route based on the router's state.
    fn to_route(&self) -> RouteInfo;

    /// Given a route info, try to resolve a child.
    fn from_route(route: &mut RouteInfo) -> Option<Self> where Self: Sized;
}

/// The top-level router at the root of the application.
/// Every possible route needs to be handled by redirecting to a 404 like page if it can't be resolved.
pub trait MainRouter: Router {
    /// Will not return an option, all cases must be handled.
    fn from_route_main(route: &mut RouteInfo) -> Self;
}


impl RouteInfo {
    /// Given either a string with a leading slash, or a full url, parse the string into a route info.
    pub fn parse(route_string: &str) -> Result<RouteInfo, RoutingError> {
        // Perform some validation on the string before parsing it.
        if let Some(first_character) = route_string.chars().next() {
            let full_url: String = if first_character == '/' {
//                eprintln!("does not start with slash: '{}'", route_string);
                format!("http://dummy_url.com{}", route_string)
//                return Err(RoutingError::RouteDoesNotStartWithSlash)
            } else {
                route_string.to_string()
            };
            Url::parse(&full_url)
                .map(RouteInfo::from)
                .map_err(|_| RoutingError::CouldNotParseRoute { route: route_string.to_string() })
        } else {
            return Err(RoutingError::RouteIsEmpty)
        }
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
