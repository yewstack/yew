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
pub trait Routable: Clone + PartialEq {
    /// Converts path to an instance of the routes enum.
    fn from_path(path: &str, params: &HashMap<&str, &str>) -> Option<Self>;

    /// Converts the route to a string that can passed to the history API.
    fn to_path(&self) -> String;

    /// Lists all the available routes
    fn routes() -> Vec<&'static str>;

    /// The route to redirect to on 404
    fn not_found_route() -> Option<Self>;

    /// Match a route based on the path
    fn recognize(pathname: &str) -> Option<Self>;
}

/// A special route that accepts any route.
///
/// This can be used with [`History`](gloo::history::History) and
/// [`Location`](gloo::history::Location) when the type of [`Routable`] is unknown.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnyRoute {
    path: String,
}

impl Routable for AnyRoute {
    fn from_path(path: &str, params: &HashMap<&str, &str>) -> Option<Self> {
        // No params allowed.
        if params.is_empty() {
            Some(Self {
                path: path.to_string(),
            })
        } else {
            None
        }
    }

    fn to_path(&self) -> String {
        self.path.to_string()
    }

    fn routes() -> Vec<&'static str> {
        vec!["/*path"]
    }

    fn not_found_route() -> Option<Self> {
        Some(Self {
            path: "/404".to_string(),
        })
    }

    fn recognize(pathname: &str) -> Option<Self> {
        Some(Self {
            path: pathname.to_string(),
        })
    }
}

impl AnyRoute {
    pub fn new<S: Into<String>>(pathname: S) -> Self {
        Self {
            path: pathname.into(),
        }
    }
}
