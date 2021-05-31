use std::collections::HashMap;

pub use yew_router_macro::Routable;

/// Marks an `enum` as routable.
///
/// # Implementation
///
/// Use derive macro to implement it. Although it *is* possible to implement it manually,
/// it is discouraged.
///
/// # Usage
///
/// The functions exposed by this trait are **not** supposed to be consumed directly. Instead use
/// the functions exposed at the [crate's root][crate] to perform operations with the router.
pub trait Routable: Sized + Clone {
    /// Converts path to an instance of the routes enum.
    fn from_path(path: &str, params: &HashMap<&str, &str>) -> Option<Self>;

    /// Converts the route to a string that can passed to the history API.
    fn to_path(&self) -> String;

    /// Lists all the available routes
    fn routes() -> Vec<&'static str>;

    /// The route to redirect to on 404
    fn not_found_route() -> Option<Self>;

    /// The current route
    ///
    /// This is the cached result of [`recognize`]
    fn current_route() -> Option<Self>;

    /// Match a route based on the path
    fn recognize(pathname: &str) -> Option<Self>;

    /// Called when [`Router`](crate::Router) is destroyed.
    fn cleanup() {}
}
