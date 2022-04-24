use crate::functional::PreparedState;

#[cfg(feature = "ssr")]
use std::future::Future;
#[cfg(feature = "ssr")]
use std::pin::Pin;
use std::rc::Rc;

use serde::de::DeserializeOwned;
use serde::Serialize;

#[cfg(feature = "hydration")]
mod feat_hydration;
#[cfg(all(feature = "hydration", feature = "ssr"))]
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

// With SSR, but no runtime available.
#[doc(hidden)]
#[cfg(all(feature = "ssr", not(any(target_arch = "wasm32", feature = "tokio"))))]
pub use yew_macro::use_prepared_state_with_closure as use_prepared_state_macro;

// With SSR, and runtime is available.
#[doc(hidden)]
#[cfg(all(feature = "ssr", any(target_arch = "wasm32", feature = "tokio")))]
pub use yew_macro::use_prepared_state_with_closure_and_suspension as use_prepared_state_macro;

// Without SSR.
#[doc(hidden)]
#[cfg(not(feature = "ssr",))]
pub use yew_macro::use_prepared_state_without_closure as use_prepared_state_macro;

/// Use a state prepared on the server side and its value is sent to client side during hydration.
///
/// The component sees the same value on the server side and client side if the component is
/// hydrated
///
/// It accepts either of the following signatures:
///
/// ```
/// # use yew::prelude::*;
/// # use serde::{Serialize, DeserializeOwned};
/// # use std::rc::Rc;
/// #[hook]
/// pub fn use_prepared_state<T, D, F>(f: F, deps: D) -> Option<Rc<T>>
/// where
///     D: Serialize + DeserializeOwned + PartialEq + 'static,
///     T: Serialize + DeserializeOwned + 'static,
///     F: FnOnce(&D) -> T,
/// # { todo!() }
/// ```
///
/// The first argument can also be an [async closure](https://github.com/rust-lang/rust/issues/62290).
/// The hook will become a suspendible hook that returns `SuspensionResult<Option<Rc<T>>>`.
///
/// ```
/// # use yew::prelude::*;
/// # use serde::{Serialize, DeserializeOwned};
/// # use std::rc::Rc;
/// # use std::future::Future;
/// #[hook]
/// pub fn use_prepared_state<T, D, F, U>(
///         f: F,
///         deps: D,
///     ) -> impl Hook<Output = SuspensionResult<Option<Rc<T>>>>
///     where
///         D: Serialize + DeserializeOwned + PartialEq + 'static,
///         T: Serialize + DeserializeOwned + 'static,
///         F: FnOnce(&D) -> U,
///         U: 'static + Future<Output = T>,
/// # { todo!() }
/// ```
///
/// If the bundle is compiled without server-side rendering, the closure will be stripped
/// automatically.
pub use use_prepared_state_macro as use_prepared_state;

#[allow(dead_code)]
struct PreparedStateBase<T, D>
where
    D: Serialize + DeserializeOwned + PartialEq + 'static,
    T: Serialize + DeserializeOwned + 'static,
{
    state: Option<Rc<T>>,
    deps: Option<Rc<D>>,
}

#[cfg(feature = "hydration")]
impl<T, D> PreparedStateBase<T, D>
where
    D: Serialize + DeserializeOwned + PartialEq + 'static,
    T: Serialize + DeserializeOwned + 'static,
{
    fn decode(buf: &[u8]) -> Self {
        let (state, deps) =
            bincode::deserialize::<(T, D)>(buf).expect("failed to deserialize state");

        PreparedStateBase {
            state: Some(Rc::new(state)),
            deps: Some(Rc::new(deps)),
        }
    }
}

impl<T, D> PreparedState for PreparedStateBase<T, D>
where
    D: Serialize + DeserializeOwned + PartialEq + 'static,
    T: Serialize + DeserializeOwned + 'static,
{
    #[cfg(feature = "ssr")]
    fn prepare(&self) -> Pin<Box<dyn Future<Output = Vec<u8>>>> {
        let state = bincode::serialize(&(self.state.as_deref(), self.deps.as_deref()))
            .expect("failed to prepare state");

        Box::pin(async move { state })
    }
}
