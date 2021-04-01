use crate::utils::get_query_params;
use crate::{Params, Routable};
use std::rc::Rc;

/// The current route.
#[derive(Clone)]
pub struct CurrentRoute {
    route: Rc<dyn Routable>,
    query: Params,
}

impl CurrentRoute {
    pub(crate) fn new(route: impl Routable + 'static) -> Self {
        Self {
            route: Rc::new(route),
            query: get_query_params().into(),
        }
    }

    /// Gets the current route
    ///
    /// # Panics
    ///
    /// Panics if `R` isn't the same [`Routable`] as the one passed to [`Router`](crate::Router) component.
    pub fn route<R: Routable + 'static>(&self) -> &R {
        self.route.as_any().downcast_ref::<R>().expect(
            "passed type argument must be the same as the one used with the `Router` component",
        )
    }

    /// Returns the query parameters from the path.
    #[inline]
    pub fn query(&self) -> &Params {
        &self.query
    }
}
