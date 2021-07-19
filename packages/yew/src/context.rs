//! This module defines the `ContextProvider` component.

use crate::html::Scope;
use crate::{html, Callback, Children, Component, Html, Properties, ShouldRender};
use slab::Slab;
use std::cell::RefCell;
use crate::html::Context;

/// Props for [`ContextProvider`]
#[derive(Debug, Clone, PartialEq, Properties)]
pub struct ContextProviderProps<T: Clone + PartialEq> {
    /// Context value to be passed down
    pub context: T,
    /// Children
    pub children: Children,
}

/// The context provider component.
///
/// Every child (direct or indirect) of this component may access the context value.
/// In order to consume contexts, [`ComponentLink::context`][Scope::context] method is used,
/// In function components the `use_context` hook is used.
#[derive(Debug)]
pub struct ContextProvider<T: Clone + PartialEq + 'static> {
    component_context: Context<Self>,
    context: T,
    children: Children,
    consumers: RefCell<Slab<Callback<T>>>,
}

/// Owns the connection to a context provider. When dropped, the component will
/// no longer receive updates from the provider.
#[derive(Debug)]
pub struct ContextHandle<T: Clone + PartialEq + 'static> {
    provider: Scope<ContextProvider<T>>,
    key: usize,
}

impl<T: Clone + PartialEq + 'static> Drop for ContextHandle<T> {
    fn drop(&mut self) {
        if let Some(component) = self.provider.get_component() {
            component.consumers.borrow_mut().remove(self.key);
        }
    }
}

impl<T: Clone + PartialEq> ContextProvider<T> {
    /// Add the callback to the subscriber list to be called whenever the context changes.
    /// The consumer is unsubscribed as soon as the callback is dropped.
    pub(crate) fn subscribe_consumer(&self, callback: Callback<T>) -> (T, ContextHandle<T>) {
        let ctx = self.context.clone();
        let key = self.consumers.borrow_mut().insert(callback);

        (
            ctx,
            ContextHandle {
                provider: self.component_context.clone(),
                key,
            },
        )
    }

    /// Notify all subscribed consumers and remove dropped consumers from the list.
    fn notify_consumers(&mut self) {
        let consumers: Vec<Callback<T>> = self
            .consumers
            .borrow()
            .iter()
            .map(|(_, v)| v.clone())
            .collect();
        for consumer in consumers {
            consumer.emit(self.context.clone());
        }
    }
}

impl<T: Clone + PartialEq + 'static> Component for ContextProvider<T> {
    type Message = ();
    type Properties = ContextProviderProps<T>;

    fn create(props: Self::Properties, ctx: &Context<Self>) -> Self {
        Self {
            component_context: ctx.clone(),
            children: props.children,
            context: props.context,
            consumers: RefCell::new(Slab::new()),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> ShouldRender { true }


    fn changed(&mut self, _ctx: &Context<Self>, props: Self::Properties) -> bool {
        let should_render = if self.children == props.children {
            false
        } else {
            self.children = props.children;
            true
        };

        if self.context != props.context {
            self.context = props.context;
            self.notify_consumers();
        }

        should_render
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! { <>{ self.children.clone() }</> }
    }
}
