use crate::utils::get_query_params;
use crate::Params;

/// The current route.
#[derive(Debug, Clone)]
pub struct CurrentRoute {
    path: String,
    params: Params,
    query: Params,
}

impl CurrentRoute {
    pub(crate) fn new(path: String, params: impl Into<Params>) -> Self {
        Self {
            path,
            params: params.into(),
            query: get_query_params().into(),
        }
    }

    /// Returns the current path.
    ///
    /// This is the `to` prop for the current [`Route`](crate::prelude::Route).
    /// If you want the current url that the user navigated to,
    /// consider using [`location::pathname()`](web_sys::Location::pathname) instead.
    #[inline]
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Returns the parameters from a path.
    ///
    /// In a path, `/path/:value`, `value` is a parameter.
    #[inline]
    pub fn parmas(&self) -> &Params {
        &self.params
    }

    /// Returns the query parameters from the path.
    #[inline]
    pub fn query(&self) -> &Params {
        &self.query
    }
}
