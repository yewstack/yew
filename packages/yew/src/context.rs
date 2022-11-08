//! This module defines the `ContextProvider` component.

use std::any::Any;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use slab::Slab;
use yew_macro::function_component;

use crate::html::AnyScope;
use crate::{
    html, use_component_id, use_effect_with_deps, use_memo, Callback, Children, Html, Properties,
};

#[derive(Debug)]
pub(crate) struct ContextStore<T: Clone + PartialEq + 'static> {
    context: T,
    consumers: Slab<Callback<T>>,
}

impl<T: Clone + PartialEq> ContextStore<T> {
    pub(crate) fn get(scope: &AnyScope) -> Option<Rc<RefCell<ContextStore<T>>>> {
        CONTEXT_STORES.with(|m| {
            m.borrow_mut()
                .get(&scope.get_id())
                .cloned()
                .and_then(|m| m.downcast().ok())
        })
    }

    /// Add the callback to the subscriber list to be called whenever the context changes.
    /// The consumer is unsubscribed as soon as the callback is dropped.
    pub(crate) fn subscribe_consumer(
        this: Rc<RefCell<Self>>,
        callback: Callback<T>,
    ) -> (T, ContextHandle<T>) {
        let (key, context) = {
            let mut this = this.borrow_mut();
            let key = this.consumers.insert(callback);
            let context = this.context.clone();

            (key, context)
        };

        (
            context,
            ContextHandle {
                provider: this,
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

    pub(crate) fn get_context_value(&self) -> T {
        self.context.clone()
    }
}

thread_local! {
    static CONTEXT_STORES: RefCell<HashMap<usize, Rc<dyn Any>>> = RefCell::default();
}

/// Props for [`ContextProvider`]
#[derive(Debug, Clone, PartialEq, Properties)]
pub struct ContextProviderProps<T: Clone + PartialEq> {
    /// Context value to be passed down
    pub context: T,
    /// Children
    pub children: Children,
}

/// Owns the connection to a context provider. When dropped, the component will
/// no longer receive updates from the provider.
#[derive(Debug)]
pub struct ContextHandle<T: Clone + PartialEq + 'static> {
    provider: Rc<RefCell<ContextStore<T>>>,
    key: usize,
}

impl<T: Clone + PartialEq + 'static> Drop for ContextHandle<T> {
    fn drop(&mut self) {
        let mut provider = self.provider.borrow_mut();
        provider.consumers.remove(self.key);
    }
}

/// The context provider component.
///
/// Every child (direct or indirect) of this component may access the context value.
/// In order to consume contexts, [`Scope::context`][Scope::context] method is used,
/// In function components the `use_context` hook is used.
#[function_component]
pub fn ContextProvider<T>(props: &ContextProviderProps<T>) -> Html
where
    T: PartialEq + Clone + 'static,
{
    let ContextProviderProps { context, children } = props.clone();
    let comp_id = use_component_id();

    {
        let context = context.clone();
        use_memo(
            |_| {
                CONTEXT_STORES.with(|m| {
                    let mut m = m.borrow_mut();

                    m.insert(
                        comp_id,
                        Rc::new(RefCell::new(ContextStore {
                            context: context.clone(),
                            consumers: Slab::new(),
                        })),
                    );
                });
            },
            (),
        );
    }

    {
        use_effect_with_deps(
            move |context| {
                let comp = CONTEXT_STORES.with(|m| {
                    m.borrow_mut()
                        .get(&comp_id)
                        .cloned()
                        .and_then(|m| m.downcast::<RefCell<ContextStore<T>>>().ok())
                        .unwrap()
                });
                let mut comp = comp.borrow_mut();

                comp.context = context.clone();
                comp.notify_consumers();
            },
            context,
        );
    }

    {
        use_effect_with_deps(
            move |_| {
                move || {
                    CONTEXT_STORES.with(|m| m.borrow_mut().remove(&comp_id));
                }
            },
            (),
        );
    }

    html! {<>{children}</>}
}
