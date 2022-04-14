#[cfg_attr(documenting, doc(cfg(any(target_arch = "wasm32", feature = "tokio"))))]
#[cfg(any(target_arch = "wasm32", feature = "tokio"))]
mod feat_futures {
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
}

#[cfg(any(target_arch = "wasm32", feature = "tokio"))]
pub use feat_futures::*;
