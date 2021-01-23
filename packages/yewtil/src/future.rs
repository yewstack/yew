use std::future::Future;
use wasm_bindgen_futures::spawn_local;
use yew::{
    agent::{Agent, AgentLink},
    Callback, Component, ComponentLink,
};

/// Trait that allows you to use `ComponentLink` and `AgentLink` to register futures.
pub trait LinkFuture {
    type Message;

    /// This method creates a `Callback` which returns a Future which
    /// returns a message to be sent back to the component's event
    /// loop.
    ///
    /// # Panics
    /// If the future panics, then the promise will not resolve, and
    /// will leak.
    fn callback_future<FN, FU, IN, M>(&self, function: FN) -> Callback<IN>
    where
        M: Into<Self::Message>,
        FU: Future<Output = M> + 'static,
        FN: Fn(IN) -> FU + 'static;

    /// This method creates a `Callback` from `FnOnce` which returns a Future
    /// which returns a message to be sent back to the component's event
    /// loop.
    ///
    /// # Panics
    /// If the future panics, then the promise will not resolve, and
    /// will leak.
    fn callback_future_once<FN, FU, IN, M>(&self, function: FN) -> Callback<IN>
    where
        M: Into<Self::Message>,
        FU: Future<Output = M> + 'static,
        FN: FnOnce(IN) -> FU + 'static;

    /// This method processes a Future that returns a message and sends it back to the component's
    /// loop.
    ///
    /// # Panics
    /// If the future panics, then the promise will not resolve, and will leak.
    fn send_future<F, M>(&self, future: F)
    where
        M: Into<Self::Message>,
        F: Future<Output = M> + 'static;

    /// Registers a future that resolves to multiple messages.
    /// # Panics
    /// If the future panics, then the promise will not resolve, and will leak.
    fn send_future_batch<F>(&self, future: F)
    where
        F: Future<Output = Vec<Self::Message>> + 'static;
}

impl<COMP: Component> LinkFuture for ComponentLink<COMP> {
    type Message = COMP::Message;

    fn callback_future<FN, FU, IN, M>(&self, function: FN) -> Callback<IN>
    where
        M: Into<Self::Message>,
        FU: Future<Output = M> + 'static,
        FN: Fn(IN) -> FU + 'static,
    {
        let link = self.clone();

        let closure = move |input: IN| {
            let future: FU = function(input);
            link.send_future(future);
        };

        closure.into()
    }

    fn callback_future_once<FN, FU, IN, M>(&self, function: FN) -> Callback<IN>
    where
        M: Into<Self::Message>,
        FU: Future<Output = M> + 'static,
        FN: FnOnce(IN) -> FU + 'static,
    {
        let link = self.clone();

        let closure = move |input: IN| {
            let future: FU = function(input);
            link.send_future(future);
        };

        Callback::once(closure)
    }

    fn send_future<F, M>(&self, future: F)
    where
        M: Into<Self::Message>,
        F: Future<Output = M> + 'static,
    {
        let link: ComponentLink<COMP> = self.clone();
        let js_future = async move {
            let message: COMP::Message = future.await.into();
            link.send_message(message);
        };
        spawn_local(js_future);
    }

    fn send_future_batch<F>(&self, future: F)
    where
        F: Future<Output = Vec<Self::Message>> + 'static,
    {
        let link: ComponentLink<COMP> = self.clone();
        let js_future = async move {
            let messages: Vec<COMP::Message> = future.await;
            link.send_message_batch(messages);
        };
        spawn_local(js_future);
    }
}

impl<AGN: Agent> LinkFuture for AgentLink<AGN> {
    type Message = AGN::Message;

    fn callback_future<FN, FU, IN, M>(&self, function: FN) -> Callback<IN>
    where
        M: Into<Self::Message>,
        FU: Future<Output = M> + 'static,
        FN: Fn(IN) -> FU + 'static,
    {
        let link = self.clone();

        let closure = move |input: IN| {
            let future: FU = function(input);
            link.send_future(future);
        };

        closure.into()
    }

    fn callback_future_once<FN, FU, IN, M>(&self, function: FN) -> Callback<IN>
    where
        M: Into<Self::Message>,
        FU: Future<Output = M> + 'static,
        FN: FnOnce(IN) -> FU + 'static,
    {
        let link = self.clone();

        let closure = move |input: IN| {
            let future: FU = function(input);
            link.send_future(future);
        };

        Callback::once(closure)
    }

    fn send_future<F, M>(&self, future: F)
    where
        M: Into<Self::Message>,
        F: Future<Output = M> + 'static,
    {
        let link: AgentLink<AGN> = self.clone();
        let js_future = async move {
            let message: AGN::Message = future.await.into();
            let cb = link.callback(|m: AGN::Message| m);
            cb.emit(message);
        };
        spawn_local(js_future);
    }

    fn send_future_batch<F>(&self, _future: F)
    where
        F: Future<Output = Vec<Self::Message>> + 'static,
    {
        unimplemented!("Agents don't support batching their messages.")
    }
}
