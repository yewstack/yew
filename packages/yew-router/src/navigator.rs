use serde::Serialize;

use crate::history::{AnyHistory, History, HistoryError, HistoryResult};
use crate::routable::Routable;

pub type NavigationError = HistoryError;
pub type NavigationResult<T> = HistoryResult<T>;

#[derive(Debug, PartialEq, Clone)]
pub struct Navigator {
    inner: AnyHistory,
}

impl Navigator {
    pub(crate) fn new(history: AnyHistory) -> Self {
        Self { inner: history }
    }

    /// Navigate back 1 page.
    pub fn back(&self) {
        self.go(-1);
    }

    /// Navigate forward 1 page in.
    pub fn forward(&self) {
        self.go(1);
    }

    /// Navigate to a specific page in with a `delta` relative to current page.
    ///
    /// See: <https://developer.mozilla.org/en-US/docs/Web/API/History/go>
    pub fn go(&self, delta: isize) {
        self.inner.go(delta);
    }

    /// Pushes a [`Routable`] entry.
    pub fn push(&self, route: impl Routable) {
        self.inner.push(route.to_path());
    }

    /// Replaces the current history entry with provided [`Routable`] and [`None`] state.
    pub fn replace(&self, route: impl Routable) {
        self.inner.replace(route.to_path());
    }

    /// Pushes a [`Routable`] entry with state.
    pub fn push_with_state<T>(&self, route: impl Routable, state: T)
    where
        T: 'static,
    {
        self.inner.push_with_state(route.to_path(), state);
    }

    /// Replaces the current history entry with provided [`Routable`] and state.
    pub fn replace_with_state<T>(&self, route: impl Routable, state: T)
    where
        T: 'static,
    {
        self.inner.replace_with_state(route.to_path(), state);
    }

    /// Same as `.push()` but affix the queries to the end of the route.
    pub fn push_with_query<Q>(&self, route: impl Routable, query: Q) -> NavigationResult<()>
    where
        Q: Serialize,
    {
        self.inner.push_with_query(route.to_path(), query)
    }

    /// Same as `.replace()` but affix the queries to the end of the route.
    pub fn replace_with_query<Q>(&self, route: impl Routable, query: Q) -> NavigationResult<()>
    where
        Q: Serialize,
    {
        self.inner.replace_with_query(route.to_path(), query)
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
            .push_with_query_and_state(route.to_path(), query, state)
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
            .replace_with_query_and_state(route.to_path(), query, state)
    }
}
