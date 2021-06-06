use crate::history::Route;

pub use yew_router_macro::Routable;

/// Marks an `enum` as routable.
///
/// # Implementation
///
/// Use derive macro to implement it. Although it *is* possible to implement it manually,
/// it is discouraged.
pub trait Routable: PartialEq + Clone + 'static {
    /// Converts the route into an instance of this type.
    /// If the conversion fails, the `Err(_)` case will be returned
    /// containing a fallback route which the caller should redirect to.
    fn from_route(route: &Route) -> Result<Self, Self>
    where
        Self: Sized;

    /// Converts an instance of this type into a route that be later turned
    /// into a URL.
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
