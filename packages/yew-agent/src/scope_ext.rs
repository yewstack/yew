//! This module contains extensions to the component scope for agent access.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

use wasm_bindgen::UnwrapThrowExt;
use yew::html::Scope;
use yew::prelude::*;

use crate::reactor::{
    Reactor, ReactorInput, ReactorOutput, ReactorReceivable, ReactorSendable, ReactorWorker,
};
use crate::task::{Task, TaskWorker};
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
        W: Worker + 'static;

    /// Bridges to a Reactor Agent.
    fn bridge_reactor<R>(
        &self,
        callback: Callback<ReactorOutput<<R::Sender as ReactorSendable>::Output>>,
    ) -> ReactorBridgeHandle<R>
    where
        R: Reactor + 'static,
        <R::Sender as ReactorSendable>::Output: 'static;

    /// Runs a task in a Task Agent.
    fn run_task<T>(&self, input: T::Input, callback: Callback<T::Output>)
    where
        T: Task + 'static;
}

impl<COMP> AgentScopeExt for Scope<COMP>
where
    COMP: Component,
{
    fn bridge_worker<W>(&self, callback: Callback<W::Output>) -> WorkerBridgeHandle<W>
    where
        W: Worker,
    {
        let inner = self
            .context::<WorkerProviderState<W>>((|_| {}).into())
            .expect_throw("failed to bridge to agent.")
            .0
            .create_bridge(move |m| callback.emit(m));

        WorkerBridgeHandle { inner }
    }

    fn bridge_reactor<R>(
        &self,
        callback: Callback<ReactorOutput<<R::Sender as ReactorSendable>::Output>>,
    ) -> ReactorBridgeHandle<R>
    where
        R: Reactor,
        <R::Sender as ReactorSendable>::Output: 'static,
    {
        let inner = self.bridge_worker::<ReactorWorker<R>>(callback).inner;

        ReactorBridgeHandle { inner }
    }

    fn run_task<T>(&self, input: T::Input, callback: Callback<T::Output>)
    where
        T: Task + 'static,
    {
        thread_local! {
            static CTR: AtomicUsize = AtomicUsize::new(0);
        }

        let task_ctr = CTR.with(|m| m.fetch_add(1, Ordering::Relaxed));

        let hold_bridge = Rc::new(RefCell::new(None));

        let bridge = {
            let hold_bridge = hold_bridge.clone();
            self.bridge_worker::<TaskWorker<T>>(
                (move |(_, output)| {
                    let hold_bridge = hold_bridge.clone();

                    callback.emit(output);

                    // Release bridge after output is emitted.
                    *hold_bridge.borrow_mut() = None;
                })
                .into(),
            )
        };

        bridge.send((task_ctr, input));

        *hold_bridge.borrow_mut() = Some(bridge);
    }
}
