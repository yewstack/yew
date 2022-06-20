//! This module contains extensions to the component scope for agent access.

use yew::html::Scope;
use yew::prelude::*;

use crate::reactor::{
    Reactor, ReactorInput, ReactorOutput, ReactorReceivable, ReactorSendable, ReactorWorker,
};
use crate::task::Task;
use crate::worker::{Worker, WorkerBridge};

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

/// A Reactor Bridge Handle.
#[derive(Debug)]
pub struct ReactorBridgeHandle<R>
where
    R: Reactor + 'static,
{
    inner: WorkerBridge<ReactorWorker<R>>,
}

impl<R> ReactorBridgeHandle<R>
where
    R: Reactor + 'static,
{
    /// Sends a message to the reactor agent.
    pub fn send(&self, input: <R::Receiver as ReactorReceivable>::Input) {
        self.inner.send(ReactorInput::Input(input))
    }
}

/// An extension to [`Scope`](yew::html::Scope) that provides communication mechanism to agents.
///
/// You can access them on `ctx.link()`
///
/// # Example
///
/// Below is an example of the implementation of the [`Link`](crate::components::Link) component.
///
/// ```
/// # use std::marker::PhantomData;
/// # use wasm_bindgen::UnwrapThrowExt;
/// # use yew::prelude::*;
/// # use yew_agent::AgentScopeExt;
/// #
/// # pub struct Link<R: Routable + 'static> {
/// #     _data: PhantomData<R>,
/// # }
/// #
/// # pub enum Msg {
/// #     OnClick,
/// # }
/// #
/// impl<R: Routable + 'static> Component for Link<R> {
///     type Message = Msg;
///     type Properties = LinkProps<R>;
///
///     fn create(_ctx: &Context<Self>) -> Self {
///         Self { _data: PhantomData }
///     }
///
///     fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
///         match msg {
///             Msg::OnClick => {
///                 ctx.link()
///                     .navigator()
///                     .expect_throw("failed to get navigator.")
///                     .push(&ctx.props().to);
///                 false
///             }
///         }
///     }
///
///     fn view(&self, ctx: &Context<Self>) -> Html {
///         html! {
///             <a class={ctx.props().classes.clone()}
///                 href={ctx.props().to.to_path()}
///                 onclick={ctx.link().callback(|e: MouseEvent| {
///                     e.prevent_default();
///                     Msg::OnClick
///                 })}
///             >
///                 { ctx.props().children.clone() }
///             </a>
///         }
///     }
/// }
/// ```
pub trait AgentScopeExt {
    /// Bridges to a Worker Agent.
    fn bridge_worker<W>(&self, callback: Callback<W::Output>) -> WorkerBridgeHandle<W>
    where
        W: Worker;

    /// Bridges to a Reactor Agent.
    fn bridge_reactor<R>(
        &self,
        callback: Callback<ReactorOutput<<R::Sender as ReactorSendable>::Output>>,
    ) -> ReactorBridgeHandle<R>
    where
        R: Reactor,
        <R::Sender as ReactorSendable>::Output: 'static;

    /// Runs a task in a Task Agent.
    fn run_task<T>(&self, callback: Callback<T::Output>)
    where
        T: Task;
}

impl<COMP> AgentScopeExt for Scope<COMP>
where
    COMP: Component,
{
    fn bridge_worker<W>(&self, callback: Callback<W::Output>) -> WorkerBridgeHandle<W>
    where
        W: Worker,
    {
        todo!()
    }

    fn bridge_reactor<R>(
        &self,
        callback: Callback<ReactorOutput<<R::Sender as ReactorSendable>::Output>>,
    ) -> ReactorBridgeHandle<R>
    where
        R: Reactor,
        <R::Sender as ReactorSendable>::Output: 'static,
    {
        todo!()
    }

    fn run_task<T>(&self, callback: Callback<T::Output>)
    where
        T: Task,
    {
        todo!()
    }
}
