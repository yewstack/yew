//! This module defines the `ContextProvider` component.

use crate::{html, Callback, Children, Component, ComponentLink, Html, Properties};
use slab::Slab;
use std::cell::RefCell;
use std::rc::Rc;

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
    link: ComponentLink<Self>,
    children: Children,
    pub(crate) context: Context<T>,
}

#[derive(Debug)]
struct ContextState<T: Clone + PartialEq + 'static> {
    value: T,
    listeners: Slab<Callback<T>>,
}

/// A context returned by `scope.context()`. This can be used to access the
/// current context value, or register a callback for when the value changes.
#[derive(Debug, Clone)]
pub struct Context<T: Clone + PartialEq + 'static> {
    state: Rc<RefCell<ContextState<T>>>,
}

impl<T: Clone + PartialEq + 'static> Context<T> {
    fn new(value: T) -> Self {
        Self {
            state: Rc::new(RefCell::new(ContextState {
                value,
                listeners: Slab::new(),
            })),
        }
    }

    /// Get the current context value.
    pub fn current(&self) -> T {
        self.state.borrow().value.clone()
    }

    /// Register a callback to be called whenever the context changes.
    /// The callback will be unregistered when the listener is dropped.
    pub fn register(&self, callback: Callback<T>) -> ContextListener<T> {
        let key = (*self.state).borrow_mut().listeners.insert(callback);
        ContextListener {
            context: self.clone(),
            key,
        }
    }
    fn store(&self, value: T) {
        let triggers = {
            let mut state = (*self.state).borrow_mut();
            if state.value != value {
                state.value = value;
                state
                    .listeners
                    .iter()
                    .map(|(_, callback)| {
                        let value = state.value.clone();
                        let callback = callback.clone();
                        move || callback.emit(value)
                    })
                    .collect()
            } else {
                Vec::new()
            }
        };

        // Call into user-code only once state is no longer borrowed.
        for trigger in triggers {
            trigger();
        }
    }
}

/// Owns the connection to a context provider. When dropped, the component will
/// no longer receive updates from the provider.
#[derive(Debug)]
pub struct ContextListener<T: Clone + PartialEq + 'static> {
    context: Context<T>,
    key: usize,
}

impl<T: Clone + PartialEq + 'static> Drop for ContextListener<T> {
    fn drop(&mut self) {
        (*self.context.state)
            .borrow_mut()
            .listeners
            .remove(self.key);
    }
}

impl<T: Clone + PartialEq + 'static> Component for ContextProvider<T> {
    type Message = ();
    type Properties = ContextProviderProps<T>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            children: props.children,
            context: Context::new(props.context),
        }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        let should_render = if self.children == props.children {
            false
        } else {
            self.children = props.children;
            true
        };

        self.context.store(props.context);

        should_render
    }

    fn view(&self) -> Html {
        html! { <>{ self.children.clone() }</> }
    }
}
