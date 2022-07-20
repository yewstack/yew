#[cfg(feature = "hydration")]
pub(super) mod feat_hydration;
#[cfg(all(feature = "hydration", feature = "ssr"))]
mod feat_hydration_ssr;
#[cfg(not(any(feature = "hydration", feature = "ssr")))]
pub(super) mod feat_none;
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
/// Use a state prepared on the server side and its value is sent to the client side during
/// hydration.
///
/// The component sees the same value on the server side and client side if the component is
/// hydrated.
///
/// It accepts a closure as the first argument and a dependency type as the second argument.
/// It returns `SuspensionResult<Option<Rc<T>>>`.
///
/// During hydration, it will only return `Ok(Some(Rc<T>))` if the component is hydrated from a
/// server-side rendering artifact and its dependency value matches.
///
/// `let state = use_prepared_state!(|deps| -> ReturnType { ... }, deps)?;`
///
/// It has the following signature:
///
/// ```
/// # use serde::de::DeserializeOwned;
/// # use serde::Serialize;
/// # use std::rc::Rc;
/// use yew::prelude::*;
/// use yew::suspense::SuspensionResult;
///
/// #[hook]
/// pub fn use_prepared_state<T, D, F>(f: F, deps: D) -> SuspensionResult<Option<Rc<T>>>
/// where
///     D: Serialize + DeserializeOwned + PartialEq + 'static,
///     T: Serialize + DeserializeOwned + 'static,
///     F: FnOnce(Rc<D>) -> T,
/// # { todo!() }
/// ```
///
/// The first argument can also be an [async closure](https://github.com/rust-lang/rust/issues/62290).
///
/// `let state = use_prepared_state!(async |deps| -> ReturnType { ... }, deps)?;`
///
/// When accepting an async closure, it has the following signature:
///
/// ```
/// # use serde::de::DeserializeOwned;
/// # use serde::Serialize;
/// # use std::rc::Rc;
/// # use std::future::Future;
/// use yew::prelude::*;
/// use yew::suspense::SuspensionResult;
///
/// #[hook]
/// pub fn use_prepared_state<T, D, F, U>(
///         f: F,
///         deps: D,
///     ) -> SuspensionResult<Option<Rc<T>>>
///     where
///         D: Serialize + DeserializeOwned + PartialEq + 'static,
///         T: Serialize + DeserializeOwned + 'static,
///         F: FnOnce(Rc<D>) -> U,
///         U: 'static + Future<Output = T>,
/// # { todo!() }
/// ```
///
/// During server-side rendering, a value of type `T` will be calculated from the first
/// closure.
///
/// If the bundle is compiled without server-side rendering, the closure will be stripped
/// automatically.
///
/// # Note
///
/// You MUST denote the return type of the closure with `|deps| -> ReturnType { ... }`. This
/// type is used during client side rendering to deserialize the state prepared on the server
/// side.
///
/// Whilst async closure is an unstable feature, the procedural macro will rewrite this to a
/// closure that returns an async block automatically. You can use this hook with async closure
/// in stable Rust.
pub use use_prepared_state_macro as use_prepared_state;
// With SSR.
#[doc(hidden)]
#[cfg(feature = "ssr")]
pub use yew_macro::use_prepared_state_with_closure as use_prepared_state_macro;
// Without SSR.
#[doc(hidden)]
#[cfg(not(feature = "ssr"))]
pub use yew_macro::use_prepared_state_without_closure as use_prepared_state_macro;

#[cfg(any(feature = "hydration", feature = "ssr"))]
mod feat_any_hydration_ssr {
    use std::marker::PhantomData;
    #[cfg(feature = "ssr")]
    use std::rc::Rc;

    use serde::de::DeserializeOwned;
    use serde::Serialize;

    use crate::functional::PreparedState;

    pub(super) struct PreparedStateBase<T, D>
    where
        D: Serialize + DeserializeOwned + PartialEq + 'static,
        T: Serialize + DeserializeOwned + 'static,
    {
        #[cfg(feature = "ssr")]
        pub state: Option<Rc<T>>,
        #[cfg(feature = "ssr")]
        pub deps: Option<Rc<D>>,
        #[cfg(feature = "hydration")]
        pub has_buf: bool,
        pub _marker: PhantomData<(T, D)>,
    }

    impl<T, D> PreparedState for PreparedStateBase<T, D>
    where
        D: Serialize + DeserializeOwned + PartialEq + 'static,
        T: Serialize + DeserializeOwned + 'static,
    {
        #[cfg(feature = "ssr")]
        fn prepare(&self) -> String {
            use base64ct::{Base64, Encoding};

            let state = bincode::serialize(&(self.state.as_deref(), self.deps.as_deref()))
                .expect("failed to prepare state");

            Base64::encode_string(&state)
        }
    }
}

#[cfg(any(feature = "hydration", feature = "ssr"))]
use feat_any_hydration_ssr::PreparedStateBase;
