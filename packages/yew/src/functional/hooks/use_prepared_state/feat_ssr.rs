//! The server-side rendering variant. This is used for server side rendering.

use std::marker::PhantomData;
use std::rc::Rc;

use serde::de::DeserializeOwned;
use serde::Serialize;

use super::PreparedStateBase;
use crate::functional::{use_memo, Hook, HookContext};

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
            let f = self.f;
            let deps = Rc::new(self.deps);

            let state = {
                let deps = deps.clone();
                use_memo(move |_| f(&deps), ()).run(ctx)
            };

            let state = PreparedStateBase {
                state: Some(state),
                deps: Some(deps),
            };

            let state =
                ctx.next_prepared_state(|_re_render, _| -> PreparedStateBase<T, D> { state });

            state.state.clone()
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
    use std::future::Future;

    use super::*;
    use crate::functional::use_state;
    use crate::io_coop::spawn_local;
    use crate::suspense::{Suspension, SuspensionResult};

    /// The with suspension variant for use_prepared_state_on_server_side.
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
                        let state_f = f(&deps);
                        spawn_local(async move {
                            let state = state_f.await;
                            result.set((Ok(Rc::new(state)), None));
                        })
                    });
                }

                let state = result.0.clone()?;

                let state = PreparedStateBase {
                    state: Some(state),
                    deps: Some(deps),
                };

                let state =
                    ctx.next_prepared_state(|_re_render, _| -> PreparedStateBase<T, D> { state });

                Ok(state.state.clone())
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
