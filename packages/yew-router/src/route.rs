//! Wrapper around route url string, and associated history state.
#[cfg(feature = "service")]
use serde::de::DeserializeOwned;

use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Debug},
    ops::Deref,
};

/// Any state that can be used in the router agent must meet the criteria of this trait.
pub trait RouteState: Serialize + DeserializeOwned + Debug + Clone + Default + 'static {}
impl<T> RouteState for T where T: Serialize + DeserializeOwned + Debug + Clone + Default + 'static {}

/// The representation of a route, segmented into different sections for easy access.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Route<STATE = ()> {
    /// The route string
    pub route: String,
    /// The state stored in the history api
    pub state: STATE,
}

impl Route<()> {
    /// Creates a new route with no state out of a string.
    ///
    /// This Route will have `()` for its state.
    pub fn new_no_state<T: AsRef<str>>(route: T) -> Self {
        Route {
            route: route.as_ref().to_string(),
            state: (),
        }
    }
}

impl<STATE: Default> Route<STATE> {
    /// Creates a new route out of a string, setting the state to its default value.
    pub fn new_default_state<T: AsRef<str>>(route: T) -> Self {
        Route {
            route: route.as_ref().to_string(),
            state: STATE::default(),
        }
    }
}

impl<STATE> fmt::Display for Route<STATE> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        std::fmt::Display::fmt(&self.route, f)
    }
}

impl<STATE> Deref for Route<STATE> {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.route
    }
}
