//! The client-side rendering variant. This is used for client side rendering.

use crate::functional::{Hook, HookContext};
use crate::hook;

use std::marker::PhantomData;
use std::rc::Rc;

use super::PreparedStateBase;
use crate::suspense::SuspensionResult;
use serde::de::DeserializeOwned;
use serde::Serialize;

/// The client-side rendering variant. This is used for client side rendering.
#[doc(hidden)]
pub fn use_prepared_state<T, D>(deps: D) -> impl Hook<Output = Option<Rc<T>>>
where
    D: Serialize + DeserializeOwned + PartialEq + 'static,
    T: Serialize + DeserializeOwned + 'static,
{
    struct HookProvider<T, D>
    where
        D: Serialize + DeserializeOwned + PartialEq + 'static,
        T: Serialize + DeserializeOwned + 'static,
    {
        _marker: PhantomData<(T, D)>,
        deps: D,
    }

    impl<T, D> Hook for HookProvider<T, D>
    where
        D: Serialize + DeserializeOwned + PartialEq + 'static,
        T: Serialize + DeserializeOwned + 'static,
    {
        type Output = Option<Rc<T>>;

        fn run(self, ctx: &mut HookContext) -> Self::Output {
            let state = ctx.next_prepared_state(|_re_render, buf| -> PreparedStateBase<T, D> {
                buf.map(|buf| PreparedStateBase::<T, D>::decode(buf))
                    .unwrap_or(PreparedStateBase {
                        state: None,
                        deps: None,
                    })
            });

            if state.deps.as_deref() == Some(&self.deps) {
                return state.state.clone();
            }

            None
        }
    }

    HookProvider::<T, D> {
        _marker: PhantomData,
        deps,
    }
}

/// The with suspension variant for use_prepared_state_on_client_side.
#[doc(hidden)]
#[hook]
pub fn use_prepared_state_with_suspension<T, D>(deps: D) -> SuspensionResult<Option<Rc<T>>>
where
    D: Serialize + DeserializeOwned + PartialEq + 'static,
    T: Serialize + DeserializeOwned + 'static,
{
    Ok(use_prepared_state(deps))
}
