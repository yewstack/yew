use std::borrow::Cow;

use crate::history::{AnyHistory, History, HistoryError, HistoryResult};
use crate::query::ToQuery;
use crate::routable::Routable;

pub type NavigationError = HistoryError;
pub type NavigationResult<T> = HistoryResult<T>;

/// The kind of Navigator Provider.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
    pub fn push<R>(&self, route: &R)
    where
        R: Routable,
    {
        self.inner.push(self.prefix_basename(&route.to_path()));
    }

    /// Replaces the current history entry with provided [`Routable`] and [`None`] state.
    pub fn replace<R>(&self, route: &R)
    where
        R: Routable,
    {
        self.inner.replace(self.prefix_basename(&route.to_path()));
    }

    /// Pushes a [`Routable`] entry with state.
    pub fn push_with_state<R, T>(&self, route: &R, state: T)
    where
        R: Routable,
        T: 'static,
    {
        self.inner
            .push_with_state(self.prefix_basename(&route.to_path()), state);
    }

    /// Replaces the current history entry with provided [`Routable`] and state.
    pub fn replace_with_state<R, T>(&self, route: &R, state: T)
    where
        R: Routable,
        T: 'static,
    {
        self.inner
            .replace_with_state(self.prefix_basename(&route.to_path()), state);
    }

    /// Same as `.push()` but affix the queries to the end of the route.
    pub fn push_with_query<R, Q>(&self, route: &R, query: Q) -> Result<(), Q::Error>
    where
        R: Routable,
        Q: ToQuery,
    {
        self.inner
            .push_with_query(self.prefix_basename(&route.to_path()), query)
    }

    /// Same as `.replace()` but affix the queries to the end of the route.
    pub fn replace_with_query<R, Q>(&self, route: &R, query: Q) -> Result<(), Q::Error>
    where
        R: Routable,
        Q: ToQuery,
    {
        self.inner
            .replace_with_query(self.prefix_basename(&route.to_path()), query)
    }

    /// Same as `.push_with_state()` but affix the queries to the end of the route.
    pub fn push_with_query_and_state<R, Q, T>(
        &self,
        route: &R,
        query: Q,
        state: T,
    ) -> Result<(), Q::Error>
    where
        R: Routable,
        Q: ToQuery,
        T: 'static,
    {
        self.inner
            .push_with_query_and_state(self.prefix_basename(&route.to_path()), query, state)
    }

    /// Same as `.replace_with_state()` but affix the queries to the end of the route.
    pub fn replace_with_query_and_state<R, Q, T>(
        &self,
        route: &R,
        query: Q,
        state: T,
    ) -> Result<(), Q::Error>
    where
        R: Routable,
        Q: ToQuery,
        T: 'static,
    {
        self.inner.replace_with_query_and_state(
            self.prefix_basename(&route.to_path()),
            query,
            state,
        )
    }

    /// Returns the Navigator kind.
    pub fn kind(&self) -> NavigatorKind {
        match &self.inner {
            AnyHistory::Browser(_) => NavigatorKind::Browser,
            AnyHistory::Hash(_) => NavigatorKind::Hash,
            AnyHistory::Memory(_) => NavigatorKind::Memory,
        }
    }

    pub(crate) fn prefix_basename<'a>(&self, route_s: &'a str) -> Cow<'a, str> {
        match self.basename() {
            Some(base) => {
                if base.is_empty() && route_s.is_empty() {
                    Cow::from("/")
                } else {
                    Cow::from(format!("{base}{route_s}"))
                }
            }
            None => route_s.into(),
        }
    }

    pub(crate) fn strip_basename<'a>(&self, path: Cow<'a, str>) -> Cow<'a, str> {
        match self.basename() {
            Some(m) => {
                let mut path = strip_basename_prefix(&path, m)
                    .map(|rest| Cow::Owned(rest.to_owned()))
                    .unwrap_or(path);

                if !path.starts_with('/') {
                    path = format!("/{path}").into();
                }

                path
            }
            None => path,
        }
    }
}

/// Strips `basename` from the start of `path`, enforcing a segment boundary.
///
/// Returns the remainder after stripping, or `None` when the prefix either
/// doesn't match or matches only a partial path segment (e.g. basename `/base`
/// must not strip `/baseball`).
fn strip_basename_prefix<'a>(path: &'a str, basename: &str) -> Option<&'a str> {
    path.strip_prefix(basename)
        .filter(|rest| rest.is_empty() || rest.starts_with('/'))
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::Navigator;
    use crate::history::{AnyHistory, MemoryHistory};

    fn make_nav(basename: Option<&str>) -> Navigator {
        Navigator::new(
            AnyHistory::from(MemoryHistory::default()),
            basename.map(str::to_owned),
        )
    }

    #[test]
    fn strip_exact_match() {
        let nav = make_nav(Some("/base"));
        assert_eq!(nav.strip_basename(Cow::from("/base")), "/");
    }

    #[test]
    fn strip_with_trailing_path() {
        let nav = make_nav(Some("/base"));
        assert_eq!(nav.strip_basename(Cow::from("/base/page")), "/page");
        assert_eq!(nav.strip_basename(Cow::from("/base/a/b/c")), "/a/b/c");
    }

    #[test]
    fn strip_partial_segment_rejected() {
        let nav = make_nav(Some("/base"));
        // `/base` must NOT be stripped from paths that merely start with those bytes
        assert_eq!(nav.strip_basename(Cow::from("/baseball")), "/baseball");
        assert_eq!(nav.strip_basename(Cow::from("/base-2")), "/base-2");
        assert_eq!(nav.strip_basename(Cow::from("/basefoo")), "/basefoo");
    }

    #[test]
    fn strip_no_match() {
        let nav = make_nav(Some("/base"));
        assert_eq!(nav.strip_basename(Cow::from("/other/page")), "/other/page");
    }

    #[test]
    fn strip_no_basename() {
        let nav = make_nav(None);
        assert_eq!(nav.strip_basename(Cow::from("/any/path")), "/any/path");
    }
}
