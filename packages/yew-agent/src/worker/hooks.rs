//! This module provides worker agent implementation.

use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

use wasm_bindgen::prelude::*;

use yew::prelude::*;

use crate::worker::provider::WorkerProviderState;
use crate::worker::{Worker, WorkerBridge};

/// State handle for the [`use_worker_bridge`] hook.
pub struct UseWorkerBridgeHandle<T>
where
    T: Worker,
{
    inner: WorkerBridge<T>,
    state: WorkerProviderState<T>,
}

impl<T> UseWorkerBridgeHandle<T>
where
    T: Worker,
{
    /// Send a message to an worker.
    pub fn send(&self, msg: T::Input) {
        self.inner.send(msg);
    }
}

impl<T> Clone for UseWorkerBridgeHandle<T>
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

impl<T> fmt::Debug for UseWorkerBridgeHandle<T>
where
    T: Worker,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UseWorkerBridgeHandle<_>")
            .field("inner", &self.inner)
            .field("state", &"_")
            .finish()
    }
}

impl<T> PartialEq for UseWorkerBridgeHandle<T>
where
    T: Worker,
{
    fn eq(&self, rhs: &Self) -> bool {
        self.state == rhs.state && self.inner == rhs.inner
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
pub fn use_worker_bridge<T, F>(on_output: F) -> UseWorkerBridgeHandle<T>
where
    T: Worker,
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

    UseWorkerBridgeHandle {
        inner: (*bridge).clone(),
        state: worker_state,
    }
}

/// State handle for the [`use_worker_subscription`] hook.
pub struct UseWorkerSubscriptionHandle<T>
where
    T: Worker,
{
    bridge: UseWorkerBridgeHandle<T>,
    outputs: Vec<Rc<T::Output>>,
    ctr: usize,
}

impl<T> UseWorkerSubscriptionHandle<T>
where
    T: Worker,
{
    /// Send an input to an worker.
    pub fn send(&self, msg: T::Input) {
        self.bridge.send(msg);
    }
}

impl<T> Clone for UseWorkerSubscriptionHandle<T>
where
    T: Worker,
{
    fn clone(&self) -> Self {
        Self {
            bridge: self.bridge.clone(),
            outputs: self.outputs.clone(),
            ctr: self.ctr,
        }
    }
}

impl<T> fmt::Debug for UseWorkerSubscriptionHandle<T>
where
    T: Worker,
    T::Output: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UseWorkerSubscriptionHandle<_>")
            .field("bridge", &self.bridge)
            .field("outputs", &self.outputs)
            .finish()
    }
}

impl<T> Deref for UseWorkerSubscriptionHandle<T>
where
    T: Worker,
{
    type Target = [Rc<T::Output>];

    fn deref(&self) -> &[Rc<T::Output>] {
        &self.outputs
    }
}

impl<T> PartialEq for UseWorkerSubscriptionHandle<T>
where
    T: Worker,
{
    fn eq(&self, rhs: &Self) -> bool {
        self.bridge == rhs.bridge && self.ctr == rhs.ctr
    }
}

/// A hook to subscribe to the outputs of a [Worker] agent.
///
/// All outputs sent to current bridge will be collected into a slice.
#[hook]
pub fn use_worker_subscription<T>() -> UseWorkerSubscriptionHandle<T>
where
    T: Worker,
{
    struct Outputs<T> {
        ctr: usize,
        inner: Vec<Rc<T>>,
    }

    impl<T> Reducible for Outputs<T> {
        type Action = Rc<T>;

        fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
            let mut outputs = self.inner.clone();

            outputs.push(action);

            Self {
                inner: outputs,
                ctr: self.ctr + 1,
            }
            .into()
        }
    }

    impl<T> Default for Outputs<T> {
        fn default() -> Self {
            Self {
                ctr: 0,
                inner: Vec::new(),
            }
        }
    }

    let outputs = use_reducer(Outputs::default);

    let bridge = {
        let outputs = outputs.clone();
        use_worker_bridge::<T, _>(move |output| outputs.dispatch(Rc::new(output)))
    };

    UseWorkerSubscriptionHandle {
        bridge,
        outputs: outputs.inner.clone(),
        ctr: outputs.ctr,
    }
}
