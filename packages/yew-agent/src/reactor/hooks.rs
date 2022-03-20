use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

use yew::prelude::*;

use super::traits::{Reactor, ReactorStation};
use super::tx_rx::{ReactorReceivable, ReactorSendable};
use crate::station::{
    use_station_bridge, use_station_subscription, UseStationBridgeHandle,
    UseStationSubscriptionHandle,
};

/// Handle for the [use_reactor_bridge] hook.
pub struct UseReactorBridgeHandle<R>
where
    R: 'static + Reactor,
{
    inner: UseStationBridgeHandle<ReactorStation<R>>,
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
        self.inner.send(msg);
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
/// The callback will be updated on every render to make sure captured values (if any) are up to date.
#[hook]
pub fn use_reactor_bridge<R, F>(on_output: F) -> UseReactorBridgeHandle<R>
where
    R: 'static + Reactor,
    F: Fn(<R::Sender as ReactorSendable>::Output) + 'static,
{
    let bridge = use_station_bridge::<ReactorStation<R>, _>(on_output);

    UseReactorBridgeHandle { inner: bridge }
}

/// Handle for the [use_reactor_subscription] hook.
pub struct UseReactorSubscriptionHandle<R>
where
    R: 'static + Reactor,
{
    inner: UseStationSubscriptionHandle<ReactorStation<R>>,
}

impl<R> Clone for UseReactorSubscriptionHandle<R>
where
    R: 'static + Reactor,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<R> UseReactorSubscriptionHandle<R>
where
    R: 'static + Reactor,
{
    /// Send an input to a reactor agent.
    pub fn send(&self, msg: <R::Receiver as ReactorReceivable>::Input) {
        self.inner.send(msg);
    }
}

impl<R> fmt::Debug for UseReactorSubscriptionHandle<R>
where
    R: Reactor,
    <R::Sender as ReactorSendable>::Output: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UseReactorSubscriptionHandle<_>")
            .field("inner", &self.inner)
            .finish()
    }
}

impl<R> Deref for UseReactorSubscriptionHandle<R>
where
    R: Reactor,
{
    type Target = [Rc<<R::Sender as ReactorSendable>::Output>];

    fn deref(&self) -> &[Rc<<R::Sender as ReactorSendable>::Output>] {
        &*self.inner
    }
}

impl<R> PartialEq for UseReactorSubscriptionHandle<R>
where
    R: Reactor,
{
    fn eq(&self, rhs: &Self) -> bool {
        self.inner == rhs.inner
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
    let sub = use_station_subscription::<ReactorStation<R>>();

    UseReactorSubscriptionHandle { inner: sub }
}
