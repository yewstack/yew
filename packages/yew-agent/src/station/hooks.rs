use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

use yew::prelude::*;

use super::traits::{Station, StationReceivable, StationWorker};
use crate::worker::{
    use_worker_bridge, use_worker_subscription, UseWorkerBridgeHandle, UseWorkerSubscriptionHandle,
};

/// Handle for the [use_station_bridge] hook.
#[derive(Debug, PartialEq)]
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

impl<S> UseStationBridgeHandle<S>
where
    S: 'static + Station,
{
    /// Send an input to a station agent.
    pub fn send(&self, msg: <S::Receivable as StationReceivable>::Input) {
        self.inner.send(msg);
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
    F: Fn(<S::Receivable as StationReceivable>::Output) + 'static,
{
    let bridge = use_worker_bridge::<StationWorker<S>, _>(on_output);

    UseStationBridgeHandle { inner: bridge }
}

/// Handle for the [use_station_subscription] hook.
#[derive(PartialEq)]
pub struct UseStationSubscriptionHandle<S>
where
    S: 'static + Station,
{
    inner: UseWorkerSubscriptionHandle<StationWorker<S>>,
}

impl<S> Clone for UseStationSubscriptionHandle<S>
where
    S: 'static + Station,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<S> UseStationSubscriptionHandle<S>
where
    S: 'static + Station,
{
    /// Send an input to a station agent.
    pub fn send(&self, msg: <S::Receivable as StationReceivable>::Input) {
        self.inner.send(msg);
    }
}

impl<S> fmt::Debug for UseStationSubscriptionHandle<S>
where
    S: Station,
    <S::Receivable as StationReceivable>::Output: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UseStationSubscriptionHandle<_>")
            .field("inner", &self.inner)
            .finish()
    }
}

impl<S> Deref for UseStationSubscriptionHandle<S>
where
    S: Station,
{
    type Target = [Rc<<S::Receivable as StationReceivable>::Output>];

    fn deref(&self) -> &[Rc<<S::Receivable as StationReceivable>::Output>] {
        &*self.inner
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
    let sub = use_worker_subscription::<StationWorker<S>>();

    UseStationSubscriptionHandle { inner: sub }
}
