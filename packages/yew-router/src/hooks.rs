//! Hooks to access router state and navigate between pages.

use crate::history::*;
use crate::routable::Routable;
use crate::router::RouterState;

use yew::prelude::*;

/// A hook to access the [`History`] type.
pub fn use_history<H>() -> Option<H>
where
    H: History + 'static,
{
    let history_state = use_context::<RouterState<H>>()?;

    Some(history_state.history())
}

/// A hook to access the [`Location`] type.
pub fn use_location<L>() -> Option<L>
where
    L: Location + 'static,
{
    Some(use_history::<L::History>()?.location())
}

/// A hook to access [`BrowserHistory`].
pub fn use_browser_history() -> Option<BrowserHistory> {
    use_history::<BrowserHistory>()
}

/// A hook to access [`BrowserLocation`].
pub fn use_browser_location() -> Option<BrowserLocation> {
    Some(use_browser_history()?.location())
}

/// A hook to access [`AnyHistory`].
pub fn use_any_history() -> Option<AnyHistory> {
    use_history::<AnyHistory>()
}

/// A hook to access [`AnyLocation`].
pub fn use_any_location() -> Option<AnyLocation> {
    use_location::<AnyLocation>()
}

/// A hook to access the current route.
///
/// This hook will return [`None`] if there's no available location or none of the routes match.
///
/// If your `Routable` has a `#[not_found]` route, you can use `.unwrap_or_default()` instead of
/// `.unwrap()` to unwrap.
pub fn use_route<R>() -> Option<R>
where
    R: Routable + 'static,
{
    use_context::<Option<R>>().and_then(|m| m)
}
