use std::borrow::Cow;

use serde::Serialize;

use crate::history::{AnyHistory, History, HistoryError, HistoryResult};
use crate::routable::Routable;

pub type NavigationError = HistoryError;
pub type NavigationResult<T> = HistoryResult<T>;

/// The kind of Navigator Provider.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum NavigatorKind {
    /// Browser History.
    Browser,
    /// Hash History.
    Hash,
    /// Memory History.
    Memory,
}

/// A struct to navigate between locations.
#[derive(Debug, PartialEq, Clone)]
pub struct Navigator {
    inner: AnyHistory,
    basename: Option<String>,
}

impl Navigator {
    pub(crate) fn new(history: AnyHistory, basename: Option<String>) -> Self {
        Self {
            inner: history,
            basename,
        }
    }

    /// Returns basename of current navigator.
    pub fn basename(&self) -> Option<&str> {
        self.basename.as_deref()
    }

    /// Navigate back 1 page.
    pub fn back(&self) {
        self.go(-1);
    }

    /// Navigate forward 1 page.
    pub fn forward(&self) {
        self.go(1);
    }

    /// Navigate to a specific page with a `delta` relative to current page.
    ///
    /// See: <https://developer.mozilla.org/en-US/docs/Web/API/History/go>
    pub fn go(&self, delta: isize) {
        self.inner.go(delta);
    }

    /// Pushes a [`Routable`] entry.
    pub fn push(&self, route: impl Routable) {
        self.inner.push(self.route_to_url(route));
    }

    /// Replaces the current history entry with provided [`Routable`] and [`None`] state.
    pub fn replace(&self, route: impl Routable) {
        self.inner.replace(self.route_to_url(route));
    }

    /// Pushes a [`Routable`] entry with state.
    pub fn push_with_state<T>(&self, route: impl Routable, state: T)
    where
        T: 'static,
    {
        self.inner.push_with_state(self.route_to_url(route), state);
    }

    /// Replaces the current history entry with provided [`Routable`] and state.
    pub fn replace_with_state<T>(&self, route: impl Routable, state: T)
    where
        T: 'static,
    {
        self.inner
            .replace_with_state(self.route_to_url(route), state);
    }

    /// Same as `.push()` but affix the queries to the end of the route.
    pub fn push_with_query<Q>(&self, route: impl Routable, query: Q) -> NavigationResult<()>
    where
        Q: Serialize,
    {
        self.inner.push_with_query(self.route_to_url(route), query)
    }

    /// Same as `.replace()` but affix the queries to the end of the route.
    pub fn replace_with_query<Q>(&self, route: impl Routable, query: Q) -> NavigationResult<()>
    where
        Q: Serialize,
    {
        self.inner
            .replace_with_query(self.route_to_url(route), query)
    }

    /// Same as `.push_with_state()` but affix the queries to the end of the route.
    pub fn push_with_query_and_state<Q, T>(
        &self,
        route: impl Routable,
        query: Q,
        state: T,
    ) -> NavigationResult<()>
    where
        Q: Serialize,
        T: 'static,
    {
        self.inner
            .push_with_query_and_state(self.route_to_url(route), query, state)
    }

    /// Same as `.replace_with_state()` but affix the queries to the end of the route.
    pub fn replace_with_query_and_state<Q, T>(
        &self,
        route: impl Routable,
        query: Q,
        state: T,
    ) -> NavigationResult<()>
    where
        Q: Serialize,
        T: 'static,
    {
        self.inner
            .replace_with_query_and_state(self.route_to_url(route), query, state)
    }

    /// Returns the Navigator kind.
    pub fn kind(&self) -> NavigatorKind {
        match &self.inner {
            AnyHistory::Browser(_) => NavigatorKind::Browser,
            AnyHistory::Hash(_) => NavigatorKind::Hash,
            AnyHistory::Memory(_) => NavigatorKind::Memory,
        }
    }

    pub(crate) fn route_to_url(&self, route: impl Routable) -> Cow<'static, str> {
        let url = route.to_path();

        let path = match self.basename() {
            Some(base) => {
                let path = format!("{}{}", base, url);
                if path.is_empty() {
                    Cow::from("/")
                } else {
                    path.into()
                }
            }
            None => url.into(),
        };

        path
    }

    pub(crate) fn strip_basename<'a>(&self, path: Cow<'a, str>) -> Cow<'a, str> {
        match self.basename() {
            Some(m) => {
                let mut path = path
                    .strip_prefix(m)
                    .map(|m| Cow::from(m.to_owned()))
                    .unwrap_or(path);

                if !path.starts_with('/') {
                    path = format!("/{}", m).into();
                }

                path
            }
            None => path,
        }
    }
}
