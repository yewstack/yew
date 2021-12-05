//! A module that provides universal session history and location information.

pub use gloo_history::{
    AnyHistory, BrowserHistory, HashHistory, History, HistoryError, HistoryResult, Location,
    MemoryHistory,
};

use crate::routable::Routable;

/// An extension to [`Location`] to provide typed routes.
pub trait LocationExt {
    /// Returns current route or `None` if none matched.
    fn route<R>(&self) -> Option<R>
    where
        R: Routable;
}

impl LocationExt for Location {
    fn route<R>(&self) -> Option<R>
    where
        R: Routable,
    {
        R::recognize(self.path())
    }
}
