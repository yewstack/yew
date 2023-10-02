//! This module contains extensions to the component scope for agent access.

use std::any::type_name;
use std::fmt;
use std::rc::Rc;

use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};
use wasm_bindgen::UnwrapThrowExt;
use yew::html::Scope;
use yew::platform::pinned::RwLock;
use yew::platform::spawn_local;
use yew::prelude::*;

use crate::oneshot::{Oneshot, OneshotProviderState};
use crate::reactor::{Reactor, ReactorBridge, ReactorEvent, ReactorProviderState, ReactorScoped};
use crate::worker::{Worker, WorkerBridge, WorkerProviderState};

/// A Worker Bridge Handle.
#[derive(Debug)]
pub struct WorkerBridgeHandle<W>
where
    W: Worker,
{
    inner: WorkerBridge<W>,
}

impl<W> WorkerBridgeHandle<W>
where
    W: Worker,
{
    /// Sends a message to the worker agent.
    pub fn send(&self, input: W::Input) {
        self.inner.send(input)
    }
}

type ReactorTx<R> =
    Rc<RwLock<SplitSink<ReactorBridge<R>, <<R as Reactor>::Scope as ReactorScoped>::Input>>>;

/// A Reactor Bridge Handle.
pub struct ReactorBridgeHandle<R>
where
    R: Reactor + 'static,
{
    tx: ReactorTx<R>,
}

impl<R> fmt::Debug for ReactorBridgeHandle<R>
where
    R: Reactor + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(type_name::<Self>()).finish_non_exhaustive()
    }
}

impl<R> ReactorBridgeHandle<R>
where
    R: Reactor + 'static,
{
    /// Sends a message to the reactor agent.
    pub fn send(&self, input: <R::Scope as ReactorScoped>::Input) {
        let tx = self.tx.clone();
        spawn_local(async move {
            let mut tx = tx.write().await;
            let _ = tx.send(input).await;
        });
    }
}

/// An extension to [`Scope`](yew::html::Scope) that provides communication mechanism to agents.
///
/// You can access them on `ctx.link()`
pub trait AgentScopeExt {
    /// Bridges to a Worker Agent.
    fn bridge_worker<W>(&self, callback: Callback<W::Output>) -> WorkerBridgeHandle<W>
    where
        W: Worker + 'static;

    /// Bridges to a Reactor Agent.
    fn bridge_reactor<R>(&self, callback: Callback<ReactorEvent<R>>) -> ReactorBridgeHandle<R>
    where
        R: Reactor + 'static,
        <R::Scope as ReactorScoped>::Output: 'static;

    /// Runs an oneshot in an Oneshot Agent.
    fn run_oneshot<T>(&self, input: T::Input, callback: Callback<T::Output>)
    where
        T: Oneshot + 'static;
}

impl<COMP> AgentScopeExt for Scope<COMP>
where
    COMP: Component,
{
    fn bridge_worker<W>(&self, callback: Callback<W::Output>) -> WorkerBridgeHandle<W>
    where
        W: Worker + 'static,
    {
        let inner = self
            .context::<Rc<WorkerProviderState<W>>>((|_| {}).into())
            .expect_throw("failed to bridge to agent.")
            .0
            .create_bridge(callback);

        WorkerBridgeHandle { inner }
    }

    fn bridge_reactor<R>(&self, callback: Callback<ReactorEvent<R>>) -> ReactorBridgeHandle<R>
    where
        R: Reactor + 'static,
        <R::Scope as ReactorScoped>::Output: 'static,
    {
        let (tx, mut rx) = self
            .context::<ReactorProviderState<R>>((|_| {}).into())
            .expect_throw("failed to bridge to agent.")
            .0
            .create_bridge()
            .split();

        spawn_local(async move {
            while let Some(m) = rx.next().await {
                callback.emit(ReactorEvent::<R>::Output(m));
            }

            callback.emit(ReactorEvent::<R>::Finished);
        });

        let tx = Rc::new(RwLock::new(tx));

        ReactorBridgeHandle { tx }
    }

    fn run_oneshot<T>(&self, input: T::Input, callback: Callback<T::Output>)
    where
        T: Oneshot + 'static,
    {
        let (inner, _) = self
            .context::<OneshotProviderState<T>>((|_| {}).into())
            .expect_throw("failed to bridge to agent.");

        spawn_local(async move { callback.emit(inner.create_bridge().run(input).await) });
    }
}
