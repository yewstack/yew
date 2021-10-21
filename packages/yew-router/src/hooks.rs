//! Hooks to access router state and navigate between pages.

use crate::history::*;
use crate::routable::Routable;
use crate::router::RouterState;

use yew::prelude::*;

/// A hook to access the [`AnyHistory`] type.
pub fn use_history() -> Option<AnyHistory> {
    let history_state = use_context::<RouterState>()?;

    Some(history_state.history())
}

/// A hook to access the [`AnyLocation`] type.
pub fn use_location() -> Option<AnyLocation> {
    Some(use_history()?.location())
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
    use_location()?.route::<R>()
}
