use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

use futures::sink::SinkExt;
use futures::stream::{SplitSink, StreamExt};
use wasm_bindgen::UnwrapThrowExt;
use yew::platform::pinned::RwLock;
use yew::platform::spawn_local;
use yew::prelude::*;

use super::provider::ReactorProviderState;
use super::{Reactor, ReactorBridge, ReactorScoped};
use crate::utils::BridgeCounter;

type ReactorTx<R> =
    Rc<RwLock<SplitSink<ReactorBridge<R>, <<R as Reactor>::Scope as ReactorScoped>::Input>>>;

/// A type that represents events from a reactor.
pub enum ReactorEvent<R>
where
    R: Reactor,
{
    /// The reactor agent has sent an output.
    Output(<R::Scope as ReactorScoped>::Output),
    /// The reactor agent has exited.
    Finished,
}

impl<R> fmt::Debug for ReactorEvent<R>
where
    R: Reactor,
    <R::Scope as ReactorScoped>::Output: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Output(m) => f.debug_tuple("ReactorEvent::Output").field(&m).finish(),
            Self::Finished => f.debug_tuple("ReactorEvent::Finished").finish(),
        }
    }
}

/// Handle for the [use_reactor_bridge] hook.
pub struct UseReactorBridgeHandle<R>
where
    R: 'static + Reactor,
{
    tx: ReactorTx<R>,
    ctr: UseReducerDispatcher<BridgeCounter>,
}

impl<R> fmt::Debug for UseReactorBridgeHandle<R>
where
    R: 'static + Reactor,
    <R::Scope as ReactorScoped>::Input: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UseReactorBridgeHandle<_>")
            .field("inner", &self.tx)
            .finish()
    }
}

impl<R> Clone for UseReactorBridgeHandle<R>
where
    R: 'static + Reactor,
{
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
            ctr: self.ctr.clone(),
        }
    }
}

impl<R> UseReactorBridgeHandle<R>
where
    R: 'static + Reactor,
{
    /// Send an input to a reactor agent.
    pub fn send(&self, msg: <R::Scope as ReactorScoped>::Input) {
        let tx = self.tx.clone();
        spawn_local(async move {
            let mut tx = tx.write().await;
            let _ = tx.send(msg).await;
        });
    }

    /// Reset the bridge.
    ///
    /// Disconnect the old bridge and re-connects the agent with a new bridge.
    pub fn reset(&self) {
        self.ctr.dispatch(());
    }
}

impl<R> PartialEq for UseReactorBridgeHandle<R>
where
    R: 'static + Reactor,
{
    fn eq(&self, rhs: &Self) -> bool {
        self.ctr == rhs.ctr
    }
}

/// A hook to bridge to a [`Reactor`].
///
/// This hooks will only bridge the reactor once over the entire component lifecycle.
///
/// Takes a callback as the argument.
///
/// The callback will be updated on every render to make sure captured values (if any) are up to
/// date.
#[hook]
pub fn use_reactor_bridge<R, F>(on_output: F) -> UseReactorBridgeHandle<R>
where
    R: 'static + Reactor,
    F: Fn(ReactorEvent<R>) + 'static,
{
    let ctr = use_reducer(BridgeCounter::default);

    let worker_state = use_context::<ReactorProviderState<R>>()
        .expect_throw("cannot find a provider for current agent.");

    let on_output = {
        let current_bridge_id = ctr.inner;
        Rc::new(move |event: ReactorEvent<R>, event_bridge_id: usize| {
            // If a new bridge is created, then we discard messages from the previous bridge.
            if current_bridge_id != event_bridge_id {
                return;
            }

            on_output(event);
        })
    };

    let on_output_ref = {
        let on_output_clone = on_output.clone();
        use_mut_ref(move || on_output_clone)
    };

    // Refresh the callback on every render.
    {
        let mut on_output_ref = on_output_ref.borrow_mut();
        *on_output_ref = on_output;
    }

    let tx = {
        use_memo((worker_state, ctr.inner), |(state, ctr)| {
            let bridge = state.create_bridge();
            let ctr = *ctr;

            let (tx, mut rx) = bridge.split();

            spawn_local(async move {
                while let Some(m) = rx.next().await {
                    let on_output = on_output_ref.borrow().clone();
                    on_output(ReactorEvent::<R>::Output(m), ctr);
                }

                let on_output = on_output_ref.borrow().clone();
                on_output(ReactorEvent::<R>::Finished, ctr);
            });

            RwLock::new(tx)
        })
    };

    UseReactorBridgeHandle {
        tx: tx.clone(),
        ctr: ctr.dispatcher(),
    }
}

/// State handle for the [`use_reactor_subscription`] hook.
pub struct UseReactorSubscriptionHandle<R>
where
    R: 'static + Reactor,
{
    bridge: UseReactorBridgeHandle<R>,
    outputs: Vec<Rc<<R::Scope as ReactorScoped>::Output>>,
    finished: bool,
    ctr: usize,
}

impl<R> UseReactorSubscriptionHandle<R>
where
    R: 'static + Reactor,
{
    /// Send an input to a reactor agent.
    pub fn send(&self, msg: <R::Scope as ReactorScoped>::Input) {
        self.bridge.send(msg);
    }

    /// Returns whether the current bridge has received a finish message.
    pub fn finished(&self) -> bool {
        self.finished
    }

    /// Reset the subscription.
    ///
    /// This disconnects the old bridge and re-connects the agent with a new bridge.
    /// Existing outputs stored in the subscription will also be cleared.
    pub fn reset(&self) {
        self.bridge.reset();
    }
}

impl<R> Clone for UseReactorSubscriptionHandle<R>
where
    R: 'static + Reactor,
{
    fn clone(&self) -> Self {
        Self {
            bridge: self.bridge.clone(),
            outputs: self.outputs.clone(),
            ctr: self.ctr,
            finished: self.finished,
        }
    }
}

impl<R> fmt::Debug for UseReactorSubscriptionHandle<R>
where
    R: 'static + Reactor,
    <R::Scope as ReactorScoped>::Input: fmt::Debug,
    <R::Scope as ReactorScoped>::Output: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UseReactorSubscriptionHandle<_>")
            .field("bridge", &self.bridge)
            .field("outputs", &self.outputs)
            .finish()
    }
}

impl<R> Deref for UseReactorSubscriptionHandle<R>
where
    R: 'static + Reactor,
{
    type Target = [Rc<<R::Scope as ReactorScoped>::Output>];

    fn deref(&self) -> &Self::Target {
        &self.outputs
    }
}

impl<R> PartialEq for UseReactorSubscriptionHandle<R>
where
    R: 'static + Reactor,
{
    fn eq(&self, rhs: &Self) -> bool {
        self.bridge == rhs.bridge && self.ctr == rhs.ctr
    }
}

/// A hook to subscribe to the outputs of a [Reactor] agent.
///
/// All outputs sent to current bridge will be collected into a slice.
#[hook]
pub fn use_reactor_subscription<R>() -> UseReactorSubscriptionHandle<R>
where
    R: 'static + Reactor,
{
    enum OutputsAction<R>
    where
        R: Reactor + 'static,
    {
        Output(ReactorEvent<R>),
        Reset,
    }

    struct Outputs<R>
    where
        R: Reactor + 'static,
    {
        ctr: usize,
        inner: Vec<Rc<<R::Scope as ReactorScoped>::Output>>,
        finished: bool,
    }

    impl<R> Reducible for Outputs<R>
    where
        R: Reactor + 'static,
    {
        type Action = OutputsAction<R>;

        fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
            let mut outputs = self.inner.clone();

            let mut finished = self.finished;

            match action {
                OutputsAction::Output(ReactorEvent::<R>::Output(m)) => outputs.push(m.into()),
                OutputsAction::Output(ReactorEvent::<R>::Finished) => {
                    finished = true;
                }
                OutputsAction::Reset => {
                    return Self {
                        inner: Vec::new(),
                        ctr: self.ctr + 1,
                        finished: false,
                    }
                    .into();
                }
            }

            Self {
                inner: outputs,
                ctr: self.ctr + 1,
                finished,
            }
            .into()
        }
    }

    impl<R> Default for Outputs<R>
    where
        R: Reactor + 'static,
    {
        fn default() -> Self {
            Self {
                ctr: 0,
                inner: Vec::new(),
                finished: false,
            }
        }
    }

    let outputs = use_reducer(Outputs::<R>::default);

    let bridge = {
        let outputs = outputs.clone();
        use_reactor_bridge::<R, _>(move |output| outputs.dispatch(OutputsAction::Output(output)))
    };

    {
        let outputs = outputs.clone();
        use_effect_with(bridge.clone(), move |_| {
            outputs.dispatch(OutputsAction::Reset);

            || {}
        });
    }

    UseReactorSubscriptionHandle {
        bridge,
        outputs: outputs.inner.clone(),
        ctr: outputs.ctr,
        finished: outputs.finished,
    }
}
