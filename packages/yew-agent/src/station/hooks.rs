use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

use yew::prelude::*;

use super::imp::StationWorker;
use super::messages::{BridgeInput, BridgeOutput};
use super::recv::StationReceivable;
use super::Station;
use crate::worker::{use_worker_bridge, UseWorkerBridgeHandle};

/// Handle for the [use_station_bridge] hook.
pub struct UseStationBridgeHandle<S>
where
    S: 'static + Station,
{
    inner: UseWorkerBridgeHandle<StationWorker<S>>,
}

impl<S> Clone for UseStationBridgeHandle<S>
where
    S: 'static + Station,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<S> fmt::Debug for UseStationBridgeHandle<S>
where
    S: 'static + Station,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UseStationBridgeHandle<_>")
            .field("inner", &self.inner)
            .finish()
    }
}

impl<S> PartialEq for UseStationBridgeHandle<S>
where
    S: 'static + Station,
{
    fn eq(&self, rhs: &Self) -> bool {
        self.inner == rhs.inner
    }
}

impl<S> UseStationBridgeHandle<S>
where
    S: 'static + Station,
{
    /// Send an input to a station agent.
    pub fn send(&self, msg: <S::Receiver as StationReceivable>::Input) {
        self.inner.send(BridgeInput::Input(msg));
    }
}

/// A hook to bridge to a [`Station`].
///
/// This hooks will only bridge the station once over the entire component lifecycle.
///
/// Takes a callback as the argument.
///
/// The callback will be updated on every render to make sure captured values (if any) are up to date.
#[hook]
pub fn use_station_bridge<S, F>(on_output: F) -> UseStationBridgeHandle<S>
where
    S: 'static + Station,
    F: Fn(BridgeOutput<<S::Receiver as StationReceivable>::Output>) + 'static,
{
    let bridge = use_worker_bridge::<StationWorker<S>, _>(on_output);

    UseStationBridgeHandle { inner: bridge }
}

/// State handle for the [`use_station_subscription`] hook.
pub struct UseStationSubscriptionHandle<S>
where
    S: 'static + Station,
{
    bridge: UseStationBridgeHandle<S>,
    outputs: Vec<Rc<<S::Receiver as StationReceivable>::Output>>,
    finished: bool,
    ctr: usize,
}

impl<S> UseStationSubscriptionHandle<S>
where
    S: 'static + Station,
{
    /// Send an input to a station agent.
    pub fn send(&self, msg: <S::Receiver as StationReceivable>::Input) {
        self.bridge.send(msg);
    }

    /// Returns whether the current stream has been finished.
    pub fn finished(&self) -> bool {
        self.finished
    }
}

impl<S> Clone for UseStationSubscriptionHandle<S>
where
    S: 'static + Station,
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

impl<S> fmt::Debug for UseStationSubscriptionHandle<S>
where
    S: 'static + Station,
    <S::Receiver as StationReceivable>::Output: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UseStationSubscriptionHandle<_>")
            .field("bridge", &self.bridge)
            .field("outputs", &self.outputs)
            .finish()
    }
}

impl<S> Deref for UseStationSubscriptionHandle<S>
where
    S: 'static + Station,
{
    type Target = [Rc<<S::Receiver as StationReceivable>::Output>];

    fn deref(&self) -> &Self::Target {
        &self.outputs
    }
}

impl<S> PartialEq for UseStationSubscriptionHandle<S>
where
    S: 'static + Station,
{
    fn eq(&self, rhs: &Self) -> bool {
        self.bridge == rhs.bridge && self.ctr == rhs.ctr
    }
}

/// A hook to subscribe to the outputs of a [Station] agent.
///
/// All outputs sent to current bridge will be collected into a slice.
#[hook]
pub fn use_station_subscription<S>() -> UseStationSubscriptionHandle<S>
where
    S: 'static + Station,
{
    struct Outputs<S>
    where
        S: Station + 'static,
    {
        ctr: usize,
        inner: Vec<Rc<<S::Receiver as StationReceivable>::Output>>,
        finished: bool,
    }

    impl<S> Reducible for Outputs<S>
    where
        S: Station + 'static,
    {
        type Action = BridgeOutput<<S::Receiver as StationReceivable>::Output>;

        fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
            let mut outputs = self.inner.clone();

            let mut finished = self.finished;

            match action {
                BridgeOutput::Output(m) => outputs.push(m.into()),
                BridgeOutput::Finish => {
                    finished = true;
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

    impl<S> Default for Outputs<S>
    where
        S: Station + 'static,
    {
        fn default() -> Self {
            Self {
                ctr: 0,
                inner: Vec::new(),
                finished: false,
            }
        }
    }

    let outputs = use_reducer(Outputs::<S>::default);

    let bridge = {
        let outputs = outputs.clone();
        use_station_bridge::<S, _>(move |output| outputs.dispatch(output))
    };

    UseStationSubscriptionHandle {
        bridge,
        outputs: outputs.inner.clone(),
        ctr: outputs.ctr,
        finished: outputs.finished,
    }
}
