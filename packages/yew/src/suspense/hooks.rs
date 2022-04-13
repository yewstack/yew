#[cfg_attr(documenting, doc(cfg(any(target_arch = "wasm32", feature = "tokio"))))]
#[cfg(any(target_arch = "wasm32", feature = "tokio"))]
mod feat_futures {
    use std::future::Future;

    use yew::prelude::*;
    use yew::suspense::{Suspension, SuspensionResult};

    /// This hook is used to await a future in a suspending context.
    ///
    /// A [Suspension] is created from the passed future and the result of the future
    /// is the output of the suspension.
    #[hook]
    pub fn use_suspending_future<T, F>(f: F) -> SuspensionResult<T>
        where
            T: Clone + 'static,
            F: Future<Output = T> + 'static,
    {
        let output = use_state(|| None);

        let suspension = {
            let output = output.clone();

            use_state(move || Suspension::from_future(async move { output.set(Some(f.await)) }))
        };

        if suspension.resumed() {
            Ok((*output).clone().unwrap())
        } else {
            Err((*suspension).clone())
        }
    }
}

#[cfg(any(target_arch = "wasm32", feature = "tokio"))]
pub use feat_futures::*;
