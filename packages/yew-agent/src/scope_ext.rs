//! This module contains extensions to the component scope for agent access.

use wasm_bindgen::UnwrapThrowExt;
use yew::html::Scope;
use yew::platform::spawn_local;
use yew::prelude::*;

// use crate::reactor::{
//     Reactor, ReactorInput, ReactorOutput, ReactorReceivable, ReactorSendable, ReactorWorker,
// };
use crate::oneshot::{Oneshot, OneshotProviderState};
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

// /// A Reactor Bridge Handle.
// #[derive(Debug)]
// pub struct ReactorBridgeHandle<R>
// where
//     R: Reactor + 'static,
// {
//     inner: WorkerBridge<ReactorWorker<R>>,
// }

// impl<R> ReactorBridgeHandle<R>
// where
//     R: Reactor + 'static,
// {
//     /// Sends a message to the reactor agent.
//     pub fn send(&self, input: <R::Receiver as ReactorReceivable>::Input) {
//         self.inner.send(ReactorInput::Input(input))
//     }
// }

/// An extension to [`Scope`](yew::html::Scope) that provides communication mechanism to agents.
///
/// You can access them on `ctx.link()`
pub trait AgentScopeExt {
    /// Bridges to a Worker Agent.
    fn bridge_worker<W>(&self, callback: Callback<W::Output>) -> WorkerBridgeHandle<W>
    where
        W: Worker + 'static;

    // /// Bridges to a Reactor Agent.
    // fn bridge_reactor<R>(
    //     &self,
    //     callback: Callback<ReactorOutput<<R::Sender as ReactorSendable>::Output>>,
    // ) -> ReactorBridgeHandle<R>
    // where
    //     R: Reactor + 'static,
    //     <R::Sender as ReactorSendable>::Output: 'static;

    /// Runs a task in a Task Agent.
    fn run_task<T>(&self, input: T::Input, callback: Callback<T::Output>)
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
            .context::<WorkerProviderState<W>>((|_| {}).into())
            .expect_throw("failed to bridge to agent.")
            .0
            .create_bridge(callback);

        WorkerBridgeHandle { inner }
    }

    // fn bridge_reactor<R>(
    //     &self,
    //     callback: Callback<ReactorOutput<<R::Sender as ReactorSendable>::Output>>,
    // ) -> ReactorBridgeHandle<R>
    // where
    //     R: Reactor,
    //     <R::Sender as ReactorSendable>::Output: 'static,
    // {
    //     let inner = self.bridge_worker::<ReactorWorker<R>>(callback).inner;

    //     ReactorBridgeHandle { inner }
    // }

    fn run_task<T>(&self, input: T::Input, callback: Callback<T::Output>)
    where
        T: Oneshot + 'static,
    {
        let (inner, _) = self
            .context::<OneshotProviderState<T>>((|_| {}).into())
            .expect_throw("failed to bridge to agent.");

        spawn_local(async move { callback.emit(inner.create_bridge().run(input).await) });
    }
}
