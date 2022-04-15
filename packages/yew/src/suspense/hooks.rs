#[cfg_attr(documenting, doc(cfg(any(target_arch = "wasm32", feature = "tokio"))))]
#[cfg(any(target_arch = "wasm32", feature = "tokio"))]
mod feat_futures {
    use std::cell::Cell;
    use std::fmt;
    use std::future::Future;
    use std::ops::Deref;

    use yew::prelude::*;
    use yew::suspense::{Suspension, SuspensionResult};

    /// This hook is used to await a future in a suspending context.
    ///
    /// A [Suspension] is created from the passed future and the result of the future
    /// is the output of the suspension.
    pub struct UseFutureHandle<O> {
        inner: UseStateHandle<Option<O>>,
    }

    impl<O> Deref for UseFutureHandle<O> {
        type Target = O;

        fn deref(&self) -> &Self::Target {
            &*self.inner.as_ref().unwrap()
        }
    }

    impl<T: fmt::Debug> fmt::Debug for UseFutureHandle<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("UseFutureHandle")
                .field("value", &format!("{:?}", self.inner))
                .finish()
        }
    }

    #[hook]
    pub fn use_future<F, T, O>(f: F) -> SuspensionResult<UseFutureHandle<O>>
    where
        F: FnOnce() -> T + 'static,
        T: Future<Output = O> + 'static,
        O: 'static,
    {
        let output = use_state(|| None);

        let suspension = {
            let output = output.clone();

            use_memo(
                move |_| {
                    Suspension::from_future(async move {
                        output.set(Some(f().await));
                    })
                },
                (),
            )
        };

        if suspension.resumed() {
            Ok(UseFutureHandle { inner: output })
        } else {
            Err((*suspension).clone())
        }
    }

    #[hook]
    pub fn use_future_with_deps<F, D, T, O>(f: F, deps: D) -> SuspensionResult<UseFutureHandle<O>>
    where
        F: FnOnce(&D) -> T + 'static,
        T: Future<Output = O> + 'static,
        O: 'static,
        D: PartialEq + 'static,
    {
        let output = use_state(|| None);
        // We only commit a result if it comes from the latest spawned future. Otherwise, this
        // might trigger pointless updates or even override newer state.
        let latest_id = use_state(|| Cell::new(0u32));

        let suspension = {
            let output = output.clone();

            use_memo(
                move |deps| {
                    let self_id = latest_id.get().wrapping_add(1);
                    // As long as less than 2**32 futures are in flight wrapping_add is fine
                    (*latest_id).set(self_id);
                    let task = f(deps);
                    Suspension::from_future(async move {
                        let result = task.await;
                        if latest_id.get() == self_id {
                            output.set(Some(result));
                        }
                    })
                },
                deps,
            )
        };

        if suspension.resumed() {
            Ok(UseFutureHandle { inner: output })
        } else {
            Err((*suspension).clone())
        }
    }
}

#[cfg(any(target_arch = "wasm32", feature = "tokio"))]
pub use feat_futures::*;
