use std::rc::Rc;

use yew::prelude::*;

use crate::primitives::{Agent, Bridge, Spawnable};

/// State handle for the [`use_private_bridge`] hook.
pub struct UsePrivateBridgeHandle<T>
where
    T: Agent,
{
    inner: Bridge<T>,
}

impl<T> UsePrivateBridgeHandle<T>
where
    T: Agent,
{
    /// Send a message to an worker.
    pub fn send(&self, msg: T::Input) {
        self.inner.send(msg);
    }
}

impl<T> Clone for UsePrivateBridgeHandle<T>
where
    T: Agent,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

/// A hook to bridge to an [`Agent`].
///
/// This hooks will only bridge the worker once over the entire component lifecycle.
///
/// Takes a callback as the first argument and a path as the second argument.
///
/// The callback will be updated on every render to make sure captured values (if any) are up to date.
#[hook]
pub fn use_private_bridge<T, F>(on_output: F, path: &str) -> UsePrivateBridgeHandle<T>
where
    T: Spawnable,
    F: Fn(T::Output) + 'static,
{
    let on_output = Rc::new(on_output);

    let on_output_clone = on_output.clone();
    let on_output_ref = use_mut_ref(move || on_output_clone);

    // Refresh the callback on every render.
    {
        let mut on_output_ref = on_output_ref.borrow_mut();
        *on_output_ref = on_output;
    }

    let bridge = use_memo(
        |path| {
            T::spawner()
                .callback(move |output| {
                    let on_output = on_output_ref.borrow().clone();
                    on_output(output);
                })
                .spawn(path)
        },
        path.to_owned(),
    );

    UsePrivateBridgeHandle {
        inner: (*bridge).clone(),
    }
}
