use std::any::type_name;
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

use crate::utils::{BridgeIdState, OutputsAction, OutputsState};
use crate::worker::provider::WorkerProviderState;
use crate::worker::{Worker, WorkerBridge};

/// Hook handle for the [`use_worker_bridge`] hook.
pub struct UseWorkerBridgeHandle<T>
where
    T: Worker,
{
    inner: Rc<WorkerBridge<T>>,
    ctr: UseReducerDispatcher<BridgeIdState>,
}

impl<T> UseWorkerBridgeHandle<T>
where
    T: Worker,
{
    /// Send an input to a worker agent.
    pub fn send(&self, msg: T::Input) {
        self.inner.send(msg);
    }

    /// Reset the bridge.
    ///
    /// Disconnect the old bridge and re-connects the agent with a new bridge.
    pub fn reset(&self) {
        self.ctr.dispatch(());
    }
}

impl<T> Clone for UseWorkerBridgeHandle<T>
where
    T: Worker,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            ctr: self.ctr.clone(),
        }
    }
}

impl<T> fmt::Debug for UseWorkerBridgeHandle<T>
where
    T: Worker,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(type_name::<Self>())
            .field("inner", &self.inner)
            .finish()
    }
}

impl<T> PartialEq for UseWorkerBridgeHandle<T>
where
    T: Worker,
{
    fn eq(&self, rhs: &Self) -> bool {
        self.inner == rhs.inner
    }
}

/// A hook to bridge to a [`Worker`].
///
/// This hooks will only bridge the worker once over the entire component lifecycle.
///
/// Takes a callback as the argument.
///
/// The callback will be updated on every render to make sure captured values (if any) are up to
/// date.
#[hook]
pub fn use_worker_bridge<T, F>(on_output: F) -> UseWorkerBridgeHandle<T>
where
    T: Worker + 'static,
    F: Fn(T::Output) + 'static,
{
    let ctr = use_reducer(BridgeIdState::default);

    let worker_state = use_context::<Rc<WorkerProviderState<T>>>()
        .expect_throw("cannot find a provider for current agent.");

    let on_output = Rc::new(on_output);

    let on_output_clone = on_output.clone();
    let on_output_ref = use_mut_ref(move || on_output_clone);

    // Refresh the callback on every render.
    {
        let mut on_output_ref = on_output_ref.borrow_mut();
        *on_output_ref = on_output;
    }

    let bridge = use_memo((worker_state, ctr.inner), |(state, _ctr)| {
        state.create_bridge(Callback::from(move |output| {
            let on_output = on_output_ref.borrow().clone();
            on_output(output);
        }))
    });

    UseWorkerBridgeHandle {
        inner: bridge,
        ctr: ctr.dispatcher(),
    }
}

/// Hook handle for the [`use_worker_subscription`] hook.
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
    /// Send an input to a worker agent.
    pub fn send(&self, msg: T::Input) {
        self.bridge.send(msg);
    }

    /// Reset the subscription.
    ///
    /// This disconnects the old bridge and re-connects the agent with a new bridge.
    /// Existing outputs stored in the subscription will also be cleared.
    pub fn reset(&self) {
        self.bridge.reset();
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
        f.debug_struct(type_name::<Self>())
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
    T: Worker + 'static,
{
    let outputs = use_reducer(OutputsState::default);

    let bridge = {
        let outputs = outputs.clone();
        use_worker_bridge::<T, _>(move |output| {
            outputs.dispatch(OutputsAction::Push(Rc::new(output)))
        })
    };

    {
        let outputs_dispatcher = outputs.dispatcher();
        use_effect_with(bridge.clone(), move |_| {
            outputs_dispatcher.dispatch(OutputsAction::Reset);

            || {}
        });
    }

    UseWorkerSubscriptionHandle {
        bridge,
        outputs: outputs.inner.clone(),
        ctr: outputs.ctr,
    }
}
