#[cfg(feature = "hydration")]
mod feat_hydration;
#[cfg(all(feature = "ssr", feature = "hydration"))]
mod feat_hydration_ssr;
#[cfg(not(any(feature = "hydration", feature = "ssr")))]
mod feat_none;
#[cfg(feature = "ssr")]
mod feat_ssr;

#[cfg(all(feature = "hydration", not(feature = "ssr")))]
pub use feat_hydration::*;
#[cfg(all(feature = "ssr", feature = "hydration"))]
pub use feat_hydration_ssr::*;
#[cfg(not(any(feature = "hydration", feature = "ssr")))]
pub use feat_none::*;
#[cfg(all(feature = "ssr", not(feature = "hydration")))]
pub use feat_ssr::*;
/// Use a state created as an artifact of the server-side rendering.
///
/// This value is created after the server-side rendering artifact is created.
///
/// It accepts a closure as the first argument and a dependency type as the second argument.
/// It returns `SuspensionResult<Option<Rc<T>>>`.
///
/// It will always return `Ok(None)` during server-side rendering.
///
/// During hydration, it will only return `Ok(Some(Rc<T>))` if the component is hydrated from a
/// server-side rendering artifact and its dependency value matches.
///
/// `let state = use_transitive_state!(|deps| -> ReturnType { ... }, deps);`
///
/// It has the following function signature:
///
/// ```
/// # use serde::de::DeserializeOwned;
/// # use serde::Serialize;
/// # use std::rc::Rc;
/// use yew::prelude::*;
/// use yew::suspense::SuspensionResult;
///
/// #[hook]
/// pub fn use_transitive_state<T, D, F>(f: F, deps: D) -> SuspensionResult<Option<Rc<T>>>
/// where
///     D: Serialize + DeserializeOwned + PartialEq + 'static,
///     T: Serialize + DeserializeOwned + 'static,
///     F: 'static + FnOnce(Rc<D>) -> T,
/// # { todo!() }
/// ```
///
/// If the bundle is compiled without server-side rendering, the closure will be stripped
/// automatically.
///
/// # Note
///
/// You MUST denote the return type of the closure with `|deps| -> ReturnType { ... }`. This
/// type is used during client side rendering to deserialize the state prepared on the server
/// side.
pub use use_transitive_state_macro as use_transitive_state;
// With SSR.
#[doc(hidden)]
#[cfg(feature = "ssr")]
pub use yew_macro::use_transitive_state_with_closure as use_transitive_state_macro;
// Without SSR.
#[doc(hidden)]
#[cfg(not(feature = "ssr"))]
pub use yew_macro::use_transitive_state_without_closure as use_transitive_state_macro;
