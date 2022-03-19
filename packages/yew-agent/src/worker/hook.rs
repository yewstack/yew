//! This module provides worker agent implementation.

use std::fmt;
use std::rc::Rc;

use wasm_bindgen::prelude::*;

use yew::prelude::*;

use crate::worker::provider::WorkerProviderState;
use crate::worker::{Bridge, Spawnable, Worker};

/// State handle for the [`use_bridge`] hook.
pub struct UseBridgeHandle<T>
where
    T: Worker,
{
    inner: Bridge<T>,
    state: WorkerProviderState<T>,
}

impl<T> UseBridgeHandle<T>
where
    T: Worker,
{
    /// Send a message to an worker.
    pub fn send(&self, msg: T::Input) {
        self.inner.send(msg);
    }
}

impl<T> Clone for UseBridgeHandle<T>
where
    T: Worker,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            state: self.state.clone(),
        }
    }
}

impl<T> fmt::Debug for UseBridgeHandle<T>
where
    T: Worker,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UseBridgeHandle<_>")
            .field("inner", &self.inner)
            .field("state", &"_")
            .finish()
    }
}

impl<T> PartialEq for UseBridgeHandle<T>
where
    T: Worker,
{
    fn eq(&self, rhs: &Self) -> bool {
        self.state == rhs.state
    }
}

/// A hook to bridge to a [`Worker`].
///
/// This hooks will only bridge the worker once over the entire component lifecycle.
///
/// Takes a callback as the argument.
///
/// The callback will be updated on every render to make sure captured values (if any) are up to date.
#[hook]
pub fn use_bridge<T, F>(on_output: F) -> UseBridgeHandle<T>
where
    T: Spawnable,
    F: Fn(T::Output) + 'static,
{
    let worker_state = use_context::<WorkerProviderState<T>>()
        .expect_throw("cannot find a provider for current agent.");

    let on_output = Rc::new(on_output);

    let on_output_clone = on_output.clone();
    let on_output_ref = use_mut_ref(move || on_output_clone);

    // Refresh the callback on every render.
    {
        let mut on_output_ref = on_output_ref.borrow_mut();
        *on_output_ref = on_output;
    }

    let bridge = use_memo(
        |state| {
            state.create_bridge(move |output| {
                let on_output = on_output_ref.borrow().clone();
                on_output(output);
            })
        },
        worker_state.clone(),
    );

    UseBridgeHandle {
        inner: (*bridge).clone(),
        state: worker_state,
    }
}
