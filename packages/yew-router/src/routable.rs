use std::collections::HashMap;

/// Marks an `enum` as routable.
///
/// # Implementation
///
/// Use derive macro to implement it. Although it *is* possible to implement it manually,
/// it is discouraged.
pub trait Routable: std::fmt::Debug {
    /// Converts path to an instance of the routes enum.
    fn from_path(path: &str, params: &HashMap<&str, &str>) -> Option<Self>
    where
        Self: Sized;

    /// Converts an instance of the routes enum to a route that can passed to browser history API.
    fn to_route(&self) -> String;

    // from https://stackoverflow.com/a/33687996
    /// Needed for downcasting done in [`CurrentRoute::route`](crate::CurrentRoute::route())
    fn as_any(&self) -> &(dyn std::any::Any + 'static);
}
