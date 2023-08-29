//! The server-side rendering variant. This is used for server side rendering.

use std::future::Future;
use std::marker::PhantomData;
use std::rc::Rc;

use serde::de::DeserializeOwned;
use serde::Serialize;

use super::PreparedStateBase;
use crate::functional::{use_memo, use_state, Hook, HookContext};
use crate::platform::spawn_local;
use crate::suspense::{Suspension, SuspensionResult};

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
            let f = self.f;
            let deps = Rc::new(self.deps);

            let state = {
                let deps = deps.clone();
                use_memo((), move |_| f(deps)).run(ctx)
            };

            let state = PreparedStateBase {
                state: Some(state),
                deps: Some(deps),
                #[cfg(feature = "hydration")]
                has_buf: true,
                _marker: PhantomData,
            };

            let state =
                ctx.next_prepared_state(|_re_render, _| -> PreparedStateBase<T, D> { state });

            Ok(state.state.clone())
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
            let f = self.f;
            let deps = Rc::new(self.deps);

            let result = use_state(|| {
                let (s, handle) = Suspension::new();
                (Err(s), Some(handle))
            })
            .run(ctx);

            {
                let deps = deps.clone();
                let result = result.clone();
                use_state(move || {
                    let state_f = f(deps.clone());

                    spawn_local(async move {
                        let state = state_f.await;
                        result.set((Ok(Rc::new(state)), None));
                    })
                })
                .run(ctx);
            }

            let state = result.0.clone()?;

            let state = PreparedStateBase {
                state: Some(state),
                deps: Some(deps),
                #[cfg(feature = "hydration")]
                has_buf: true,
                _marker: PhantomData,
            };

            let state =
                ctx.next_prepared_state(|_re_render, _| -> PreparedStateBase<T, D> { state });

            Ok(state.state.clone())
        }
    }

    HookProvider::<T, D, F, U> { deps, f }
}
