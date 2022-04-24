//! The client-and-server-side rendering variant.

use crate::functional::{Hook, HookContext};
use crate::html::RenderMode;

use std::rc::Rc;

use super::feat_hydration;
use super::feat_ssr;
use crate::suspense::SuspensionResult;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::marker::PhantomData;

#[doc(hidden)]
pub fn use_prepared_state<T, D, F>(f: F, deps: D) -> impl Hook<Output = Option<Rc<T>>>
where
    D: Serialize + DeserializeOwned + PartialEq + 'static,
    T: Serialize + DeserializeOwned + 'static,
    F: FnOnce(&D) -> T,
{
    struct HookProvider<T, D, F>
    where
        D: Serialize + DeserializeOwned + PartialEq + 'static,
        T: Serialize + DeserializeOwned + 'static,
        F: FnOnce(&D) -> T,
    {
        _marker: PhantomData<(T, D)>,
        deps: D,
        f: F,
    }

    impl<T, D, F> Hook for HookProvider<T, D, F>
    where
        D: Serialize + DeserializeOwned + PartialEq + 'static,
        T: Serialize + DeserializeOwned + 'static,
        F: FnOnce(&D) -> T,
    {
        type Output = Option<Rc<T>>;

        fn run(self, ctx: &mut HookContext) -> Self::Output {
            match ctx.mode {
                RenderMode::Ssr => feat_ssr::use_prepared_state(self.f, self.deps).run(ctx),
                _ => feat_hydration::use_prepared_state(self.deps).run(ctx),
            }
        }
    }

    HookProvider::<T, D, F> {
        _marker: PhantomData,
        deps,
        f,
    }
}

#[cfg_attr(documenting, doc(cfg(any(target_arch = "wasm32", feature = "tokio"))))]
#[cfg(any(target_arch = "wasm32", feature = "tokio"))]
mod feat_io {
    use super::*;
    use std::future::Future;

    #[doc(hidden)]
    pub fn use_prepared_state_with_suspension<T, D, F, U>(
        f: F,
        deps: D,
    ) -> impl Hook<Output = SuspensionResult<Option<Rc<T>>>>
    where
        D: Serialize + DeserializeOwned + PartialEq + 'static,
        T: Serialize + DeserializeOwned + 'static,
        F: FnOnce(&D) -> U,
        U: 'static + Future<Output = T>,
    {
        struct HookProvider<T, D, F, U>
        where
            D: Serialize + DeserializeOwned + PartialEq + 'static,
            T: Serialize + DeserializeOwned + 'static,
            F: FnOnce(&D) -> U,
            U: 'static + Future<Output = T>,
        {
            _marker: PhantomData<(T, D)>,
            deps: D,
            f: F,
        }

        impl<T, D, F, U> Hook for HookProvider<T, D, F, U>
        where
            D: Serialize + DeserializeOwned + PartialEq + 'static,
            T: Serialize + DeserializeOwned + 'static,
            F: FnOnce(&D) -> U,
            U: 'static + Future<Output = T>,
        {
            type Output = SuspensionResult<Option<Rc<T>>>;

            fn run(self, ctx: &mut HookContext) -> Self::Output {
                match ctx.mode {
                    RenderMode::Ssr => {
                        feat_ssr::use_prepared_state_with_suspension(self.f, self.deps).run(ctx)
                    }
                    _ => feat_hydration::use_prepared_state_with_suspension(self.deps).run(ctx),
                }
            }
        }

        HookProvider::<T, D, F, U> {
            _marker: PhantomData,
            deps,
            f,
        }
    }
}

#[cfg_attr(documenting, doc(cfg(any(target_arch = "wasm32", feature = "tokio"))))]
#[cfg(any(target_arch = "wasm32", feature = "tokio"))]
pub use feat_io::*;
