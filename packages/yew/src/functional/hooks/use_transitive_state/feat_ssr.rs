//! The server-side rendering variant.

use std::cell::RefCell;
use std::rc::Rc;

use base64ct::{Base64, Encoding};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::functional::{Hook, HookContext, PreparedState};
use crate::suspense::SuspensionResult;

pub(super) struct TransitiveStateBase<T, D, F>
where
    D: Serialize + DeserializeOwned + PartialEq + 'static,
    T: Serialize + DeserializeOwned + 'static,
    F: 'static + FnOnce(Rc<D>) -> T,
{
    pub state_fn: RefCell<Option<F>>,
    pub deps: Rc<D>,
}

impl<T, D, F> PreparedState for TransitiveStateBase<T, D, F>
where
    D: Serialize + DeserializeOwned + PartialEq + 'static,
    T: Serialize + DeserializeOwned + 'static,
    F: 'static + FnOnce(Rc<D>) -> T,
{
    fn prepare(&self) -> String {
        let f = self.state_fn.borrow_mut().take().unwrap();
        let state = f(self.deps.clone());

        let state = bincode::serialize(&(Some(&state), Some(&*self.deps)))
            .expect("failed to prepare state");

        Base64::encode_string(&state)
    }
}

#[doc(hidden)]
pub fn use_transitive_state<T, D, F>(
    deps: D,
    f: F,
) -> impl Hook<Output = SuspensionResult<Option<Rc<T>>>>
where
    D: Serialize + DeserializeOwned + PartialEq + 'static,
    T: Serialize + DeserializeOwned + 'static,
    F: 'static + FnOnce(Rc<D>) -> T,
{
    struct HookProvider<T, D, F>
    where
        D: Serialize + DeserializeOwned + PartialEq + 'static,
        T: Serialize + DeserializeOwned + 'static,
        F: 'static + FnOnce(Rc<D>) -> T,
    {
        deps: D,
        f: F,
    }

    impl<T, D, F> Hook for HookProvider<T, D, F>
    where
        D: Serialize + DeserializeOwned + PartialEq + 'static,
        T: Serialize + DeserializeOwned + 'static,
        F: 'static + FnOnce(Rc<D>) -> T,
    {
        type Output = SuspensionResult<Option<Rc<T>>>;

        fn run(self, ctx: &mut HookContext) -> Self::Output {
            let f = self.f;

            ctx.next_prepared_state(move |_re_render, _| -> TransitiveStateBase<T, D, F> {
                TransitiveStateBase {
                    state_fn: Some(f).into(),
                    deps: self.deps.into(),
                }
            });

            Ok(None)
        }
    }

    HookProvider::<T, D, F> { deps, f }
}
