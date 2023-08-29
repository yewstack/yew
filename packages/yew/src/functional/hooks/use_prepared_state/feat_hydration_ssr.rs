//! The client-and-server-side rendering variant.

use std::future::Future;
use std::rc::Rc;

use serde::de::DeserializeOwned;
use serde::Serialize;

use super::{feat_hydration, feat_ssr};
use crate::functional::{Hook, HookContext};
use crate::html::RenderMode;
use crate::suspense::SuspensionResult;

#[doc(hidden)]
pub fn use_prepared_state<T, D, F>(
    deps: D,
    f: F,
) -> impl Hook<Output = SuspensionResult<Option<Rc<T>>>>
where
    D: Serialize + DeserializeOwned + PartialEq + 'static,
    T: Serialize + DeserializeOwned + 'static,
    F: FnOnce(Rc<D>) -> T,
{
    struct HookProvider<T, D, F>
    where
        D: Serialize + DeserializeOwned + PartialEq + 'static,
        T: Serialize + DeserializeOwned + 'static,
        F: FnOnce(Rc<D>) -> T,
    {
        deps: D,
        f: F,
    }

    impl<T, D, F> Hook for HookProvider<T, D, F>
    where
        D: Serialize + DeserializeOwned + PartialEq + 'static,
        T: Serialize + DeserializeOwned + 'static,
        F: FnOnce(Rc<D>) -> T,
    {
        type Output = SuspensionResult<Option<Rc<T>>>;

        fn run(self, ctx: &mut HookContext) -> Self::Output {
            match ctx.creation_mode {
                RenderMode::Ssr => feat_ssr::use_prepared_state(self.deps, self.f).run(ctx),
                _ => feat_hydration::use_prepared_state(self.deps).run(ctx),
            }
        }
    }

    HookProvider::<T, D, F> { deps, f }
}

#[doc(hidden)]
pub fn use_prepared_state_with_suspension<T, D, F, U>(
    deps: D,
    f: F,
) -> impl Hook<Output = SuspensionResult<Option<Rc<T>>>>
where
    D: Serialize + DeserializeOwned + PartialEq + 'static,
    T: Serialize + DeserializeOwned + 'static,
    F: FnOnce(Rc<D>) -> U,
    U: 'static + Future<Output = T>,
{
    struct HookProvider<T, D, F, U>
    where
        D: Serialize + DeserializeOwned + PartialEq + 'static,
        T: Serialize + DeserializeOwned + 'static,
        F: FnOnce(Rc<D>) -> U,
        U: 'static + Future<Output = T>,
    {
        deps: D,
        f: F,
    }

    impl<T, D, F, U> Hook for HookProvider<T, D, F, U>
    where
        D: Serialize + DeserializeOwned + PartialEq + 'static,
        T: Serialize + DeserializeOwned + 'static,
        F: FnOnce(Rc<D>) -> U,
        U: 'static + Future<Output = T>,
    {
        type Output = SuspensionResult<Option<Rc<T>>>;

        fn run(self, ctx: &mut HookContext) -> Self::Output {
            match ctx.creation_mode {
                RenderMode::Ssr => {
                    feat_ssr::use_prepared_state_with_suspension(self.deps, self.f).run(ctx)
                }
                _ => feat_hydration::use_prepared_state(self.deps).run(ctx),
            }
        }
    }

    HookProvider::<T, D, F, U> { deps, f }
}
