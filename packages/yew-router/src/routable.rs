use std::collections::HashMap;

pub use yew_router_macro::Routable;

/// Marks an `enum` as routable.
///
/// # Implementation
///
/// Use derive macro to implement it. Although it *is* possible to implement it manually,
/// it is discouraged.
pub trait Routable {
    /// Converts path to an instance of the routes enum.
    fn from_path(path: &str, params: &HashMap<&str, &str>) -> Option<Self>
    where
        Self: Sized;

    /// Converts an instance of the routes enum to a route that can passed to browser history API.
    fn to_route(&self) -> String;

    /// Lists all the available routes
    fn routes() -> Vec<&'static str>;

    /// The route to redirect to on 404
    fn not_found_route() -> Option<&'static str>;

    /// Match a route based on the path
    fn recognize(pathname: &str) -> Option<Self>
    where
        Self: Sized;
}
