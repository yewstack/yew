//! This module provides worker agent implementation.

use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

use crate::worker::provider::WorkerProviderState;
use crate::worker::{Worker, WorkerBridge};

#[derive(Default)]
struct UseBridgeCounter {
    inner: usize,
}

impl Reducible for UseBridgeCounter {
    type Action = ();

    fn reduce(self: Rc<Self>, _: Self::Action) -> Rc<Self> {
        Self {
            inner: self.inner + 1,
        }
        .into()
    }
}

/// State handle for the [`use_worker_bridge`] hook.
pub struct UseWorkerBridgeHandle<T>
where
    T: Worker,
{
    inner: WorkerBridge<T>,
    ctr: UseReducerDispatcher<UseBridgeCounter>,
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
        f.debug_struct("UseWorkerBridgeHandle<_>")
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
    T: Worker,
    F: Fn(T::Output) + 'static,
{
    let ctr = use_reducer(UseBridgeCounter::default);

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
        |(state, _ctr)| {
            state.create_bridge(Callback::from(move |output| {
                let on_output = on_output_ref.borrow().clone();
                on_output(output);
            }))
        },
        (worker_state, ctr.inner),
    );

    UseWorkerBridgeHandle {
        inner: (*bridge).clone(),
        ctr: ctr.dispatcher(),
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
    enum OutputsAction<T> {
        Push(Rc<T>),
        Reset,
    }

    struct Outputs<T> {
        ctr: usize,
        inner: Vec<Rc<T>>,
    }

    impl<T> Reducible for Outputs<T> {
        type Action = OutputsAction<T>;

        fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
            match action {
                Self::Action::Push(m) => {
                    let mut outputs = self.inner.clone();

                    outputs.push(m);

                    Self {
                        inner: outputs,
                        ctr: self.ctr + 1,
                    }
                    .into()
                }
                Self::Action::Reset => Self {
                    inner: Vec::new(),
                    ctr: self.ctr + 1,
                }
                .into(),
            }
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
        use_worker_bridge::<T, _>(move |output| {
            outputs.dispatch(OutputsAction::Push(Rc::new(output)))
        })
    };

    {
        let outputs_dispatcher = outputs.dispatcher();
        use_effect_with_deps(
            move |_| {
                outputs_dispatcher.dispatch(OutputsAction::Reset);

                || {}
            },
            bridge.clone(),
        );
    }

    UseWorkerSubscriptionHandle {
        bridge,
        outputs: outputs.inner.clone(),
        ctr: outputs.ctr,
    }
}
