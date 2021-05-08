use crate::agents::history::Route;

pub use yew_router_macro::Routable;

/// Marks an `enum` as routable.
///
/// # Implementation
///
/// Use derive macro to implement it. Although it *is* possible to implement it manually,
/// it is discouraged.
pub trait Routable: PartialEq + 'static {
    /// Converts path to an instance of the routes enum. If the conversion succeeds, the
    /// returned variant corresponds to the route. If the conversion fails, a fallback variant
    /// is returned, which the caller should redirect to.
    fn from_route(route: &Route) -> Result<Self, Self>
    where
        Self: Sized;

    /// Converts an instance of the routes enum to a route that can passed to browser history API.
    fn to_route(&self) -> Route;
}

impl Routable for Route {
    fn from_route(route: &Route) -> Result<Self, Self>
    where
        Self: Sized,
    {
        Ok(route.clone())
    }

    fn to_route(&self) -> Route {
        self.clone()
    }
}
