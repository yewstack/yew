use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

use yew::prelude::*;

use super::messages::{ReactorInput, ReactorOutput};
use super::traits::{Reactor, ReactorWorker};
use super::tx_rx::{ReactorReceivable, ReactorSendable};
use crate::worker::{use_worker_bridge, UseWorkerBridgeHandle};

/// Handle for the [use_reactor_bridge] hook.
pub struct UseReactorBridgeHandle<R>
where
    R: 'static + Reactor,
{
    inner: UseWorkerBridgeHandle<ReactorWorker<R>>,
}

impl<R> fmt::Debug for UseReactorBridgeHandle<R>
where
    R: 'static + Reactor,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UseReactorBridgeHandle<_>")
            .field("inner", &self.inner)
            .finish()
    }
}

impl<R> Clone for UseReactorBridgeHandle<R>
where
    R: 'static + Reactor,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<R> UseReactorBridgeHandle<R>
where
    R: 'static + Reactor,
{
    /// Send an input to a reactor agent.
    pub fn send(&self, msg: <R::Receiver as ReactorReceivable>::Input) {
        self.inner.send(ReactorInput::Input(msg));
    }

    /// Reset the bridge.
    ///
    /// Disconnect the old bridge and re-connects the agent with a new bridge.
    pub fn reset(&self) {
        self.inner.reset();
    }
}

impl<R> PartialEq for UseReactorBridgeHandle<R>
where
    R: 'static + Reactor,
{
    fn eq(&self, rhs: &Self) -> bool {
        self.inner == rhs.inner
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
    F: Fn(ReactorOutput<<R::Sender as ReactorSendable>::Output>) + 'static,
{
    let bridge = use_worker_bridge::<ReactorWorker<R>, _>(on_output);

    {
        let bridge = bridge.clone();

        use_effect(move || {
            bridge.send(ReactorInput::Start);
            || {}
        });
    }

    UseReactorBridgeHandle { inner: bridge }
}

/// State handle for the [`use_reactor_subscription`] hook.
pub struct UseReactorSubscriptionHandle<R>
where
    R: 'static + Reactor,
{
    bridge: UseReactorBridgeHandle<R>,
    outputs: Vec<Rc<<R::Sender as ReactorSendable>::Output>>,
    finished: bool,
    ctr: usize,
}

impl<R> UseReactorSubscriptionHandle<R>
where
    R: 'static + Reactor,
{
    /// Send an input to a reactor agent.
    pub fn send(&self, msg: <R::Receiver as ReactorReceivable>::Input) {
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
    <R::Sender as ReactorSendable>::Output: fmt::Debug,
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
    type Target = [Rc<<R::Sender as ReactorSendable>::Output>];

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
        Output(ReactorOutput<<R::Sender as ReactorSendable>::Output>),
        Reset,
    }

    struct Outputs<R>
    where
        R: Reactor + 'static,
    {
        ctr: usize,
        inner: Vec<Rc<<R::Sender as ReactorSendable>::Output>>,
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
                OutputsAction::Output(ReactorOutput::Output(m)) => outputs.push(m.into()),
                OutputsAction::Output(ReactorOutput::Finish) => {
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
        use_effect_with_deps(
            move |_| {
                outputs.dispatch(OutputsAction::Reset);

                || {}
            },
            bridge.clone(),
        );
    }

    UseReactorSubscriptionHandle {
        bridge,
        outputs: outputs.inner.clone(),
        ctr: outputs.ctr,
        finished: outputs.finished,
    }
}
