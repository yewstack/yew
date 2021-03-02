//! Parses routes into enums or structs.
use crate::route::Route;
use std::fmt::Write;

/// Alias to Switch.
///
/// Eventually Switch will be renamed to Routable and this alias will be removed.
#[allow(bare_trait_objects)]
pub type Routable = Switch;

/// Derivable routing trait that allows instances of implementors to be constructed from Routes.
///
/// # Note
/// Don't try to implement this yourself, rely on the derive macro.
///
/// # Example
/// ```
/// use yew_router::{route::Route, Switch};
/// #[derive(Debug, Switch, PartialEq)]
/// enum TestEnum {
///     #[at = "/test/route"]
///     TestRoute,
///     #[at = "/capture/string/{path}"]
///     CaptureString { path: String },
///     #[at = "/capture/number/{num}"]
///     CaptureNumber { num: usize },
///     #[at = "/capture/unnamed/{doot}"]
///     CaptureUnnamed(String),
/// }
///
/// assert_eq!(
///     TestEnum::switch(Route::new_no_state("/test/route")),
///     Some(TestEnum::TestRoute)
/// );
/// assert_eq!(
///     TestEnum::switch(Route::new_no_state("/capture/string/lorem")),
///     Some(TestEnum::CaptureString {
///         path: "lorem".to_string()
///     })
/// );
/// assert_eq!(
///     TestEnum::switch(Route::new_no_state("/capture/number/22")),
///     Some(TestEnum::CaptureNumber { num: 22 })
/// );
/// assert_eq!(
///     TestEnum::switch(Route::new_no_state("/capture/unnamed/lorem")),
///     Some(TestEnum::CaptureUnnamed("lorem".to_string()))
/// );
/// ```
pub trait Switch: Sized {
    /// Based on a route, possibly produce an itself.
    fn switch<STATE>(route: Route<STATE>) -> Option<Self> {
        Self::from_route_part(route.route, Some(route.state)).0
    }

    /// Get self from a part of the state
    fn from_route_part<STATE>(part: String, state: Option<STATE>) -> (Option<Self>, Option<STATE>);

    /// Build part of a route from itself.
    fn build_route_section<STATE>(self, route: &mut String) -> Option<STATE>;

    /// Called when the key (the named capture group) can't be located. Instead of failing outright,
    /// a default item can be provided instead.
    ///
    /// Its primary motivation for existing is to allow implementing Switch for Option.
    /// This doesn't make sense at the moment because this only works for the individual key section
    /// - any surrounding literals are pretty much guaranteed to make the parse step fail.
    /// because of this, this functionality might be removed in favor of using a nested Switch enum,
    /// or multiple variants.
    fn key_not_available() -> Option<Self> {
        None
    }
}

/// Wrapper that requires that an implementor of Switch must start with a `/`.
///
/// This is needed for any non-derived type provided by yew-router to be used by itself.
///
/// This is because route strings will almost always start with `/`, so in order to get a std type
/// with the `rest` attribute, without a specified leading `/`, this wrapper is needed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LeadingSlash<T>(pub T);
impl<U: Switch> Switch for LeadingSlash<U> {
    fn from_route_part<STATE>(part: String, state: Option<STATE>) -> (Option<Self>, Option<STATE>) {
        if let Some(part) = part.strip_prefix('/') {
            let (inner, state) = U::from_route_part(part.to_owned(), state);
            (inner.map(LeadingSlash), state)
        } else {
            (None, None)
        }
    }

    fn build_route_section<T>(self, route: &mut String) -> Option<T> {
        write!(route, "/").ok()?;
        self.0.build_route_section(route)
    }
}

/// Successfully match even when the captured section can't be found.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Permissive<U>(pub Option<U>);

impl<U: Switch> Switch for Permissive<U> {
    /// Option is very permissive in what is allowed.
    fn from_route_part<STATE>(part: String, state: Option<STATE>) -> (Option<Self>, Option<STATE>) {
        let (inner, inner_state) = U::from_route_part(part, state);
        if inner.is_some() {
            (Some(Permissive(inner)), inner_state)
        } else {
            // The Some(None) here indicates that this will produce a None, if the wrapped value can't be parsed
            (Some(Permissive(None)), None)
        }
    }

    fn build_route_section<STATE>(self, route: &mut String) -> Option<STATE> {
        if let Some(inner) = self.0 {
            inner.build_route_section(route)
        } else {
            None
        }
    }

    fn key_not_available() -> Option<Self> {
        Some(Permissive(None))
    }
}

// TODO the AllowMissing shim doesn't appear to offer much over Permissive.
// Documentation should improve (need examples - to show the difference) or it should be removed.

/// Allows a section to match, providing a None value,
/// if its contents are entirely missing, or starts with a '/'.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct AllowMissing<U: std::fmt::Debug>(pub Option<U>);
impl<U: Switch + std::fmt::Debug> Switch for AllowMissing<U> {
    fn from_route_part<STATE>(part: String, state: Option<STATE>) -> (Option<Self>, Option<STATE>) {
        let route = part.clone();
        let (inner, inner_state) = U::from_route_part(part, state);

        if inner.is_some() {
            (Some(AllowMissing(inner)), inner_state)
        } else if route.is_empty()
            || route.starts_with('/')
            || route.starts_with('?')
            || route.starts_with('&')
            || route.starts_with('#')
        {
            (Some(AllowMissing(None)), inner_state)
        } else {
            (None, None)
        }
    }

    fn build_route_section<STATE>(self, route: &mut String) -> Option<STATE> {
        if let AllowMissing(Some(inner)) = self {
            inner.build_route_section(route)
        } else {
            None
        }
    }
}

/// Builds a route from a switch.
fn build_route_from_switch<SW: Switch, STATE: Default>(switch: SW) -> Route<STATE> {
    // URLs are recommended to not be over 255 characters,
    // although browsers frequently support up to about 2000.
    // Routes, being a subset of URLs should probably be smaller than 255 characters for the vast
    // majority of circumstances, preventing reallocation under most conditions.
    let mut buf = String::with_capacity(255);
    let state: STATE = switch.build_route_section(&mut buf).unwrap_or_default();
    buf.shrink_to_fit();

    Route { route: buf, state }
}

impl<SW: Switch, STATE: Default> From<SW> for Route<STATE> {
    fn from(switch: SW) -> Self {
        build_route_from_switch(switch)
    }
}

impl<T: std::str::FromStr + std::fmt::Display> Switch for T {
    fn from_route_part<U>(part: String, state: Option<U>) -> (Option<Self>, Option<U>) {
        (::std::str::FromStr::from_str(&part).ok(), state)
    }

    fn build_route_section<U>(self, route: &mut String) -> Option<U> {
        write!(route, "{}", self).expect("Writing to string should never fail.");
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn isize_build_route() {
        let mut route = "/".to_string();
        let mut _state: Option<String> = None;
        _state = _state.or_else(|| (-432isize).build_route_section(&mut route));
        assert_eq!(route, "/-432".to_string());
    }

    #[test]
    fn can_get_string_from_empty_str() {
        let (s, _state) = String::from_route_part::<()>("".to_string(), Some(()));
        assert_eq!(s, Some("".to_string()))
    }

    #[test]
    fn uuid_from_route() {
        let x = uuid::Uuid::switch::<()>(Route {
            route: "5dc48134-35b5-4b8c-aa93-767bf00ae1d8".to_string(),
            state: (),
        });
        assert!(x.is_some())
    }
    #[test]
    fn uuid_to_route() {
        use std::str::FromStr;
        let id =
            uuid::Uuid::from_str("5dc48134-35b5-4b8c-aa93-767bf00ae1d8").expect("should parse");
        let mut buf = String::new();
        id.build_route_section::<()>(&mut buf);
        assert_eq!(buf, "5dc48134-35b5-4b8c-aa93-767bf00ae1d8".to_string())
    }

    #[test]
    fn can_get_option_string_from_empty_str() {
        let (s, _state): (Option<Permissive<String>>, Option<()>) =
            Permissive::from_route_part("".to_string(), Some(()));
        assert_eq!(s, Some(Permissive(Some("".to_string()))))
    }
}
