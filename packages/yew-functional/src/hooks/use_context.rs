use super::{use_hook, Hook};
use crate::get_current_scope;
use std::any::TypeId;
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::{iter, mem};
use yew::html;
use yew::html::{AnyScope, Scope};
use yew::{Children, Component, ComponentLink, Html, Properties};

type ConsumerCallback<T> = Box<dyn Fn(Rc<T>)>;

/// Props for [`ContextProvider`]
#[derive(Clone, PartialEq, Properties)]
pub struct ContextProviderProps<T: Clone + PartialEq> {
    pub context: T,
    pub children: Children,
}

/// The context provider component.
///
/// Every child (direct or indirect) of this component may access the context value.
/// Currently the only way to consume the context is using the [`use_context`] hook.
pub struct ContextProvider<T: Clone + PartialEq + 'static> {
    context: Rc<T>,
    children: Children,
    consumers: RefCell<Vec<Weak<ConsumerCallback<T>>>>,
}

impl<T: Clone + PartialEq> ContextProvider<T> {
    /// Add the callback to the subscriber list to be called whenever the context changes.
    /// The consumer is unsubscribed as soon as the callback is dropped.
    fn subscribe_consumer(&self, mut callback: Weak<ConsumerCallback<T>>) {
        let mut consumers = self.consumers.borrow_mut();
        // consumers re-subscribe on every render. Try to keep the subscriber list small by reusing dead slots.
        for cb in consumers.iter_mut() {
            if cb.strong_count() == 0 {
                mem::swap(cb, &mut callback);
                return;
            }
        }

        // no slot to reuse, this is a new consumer
        consumers.push(callback);
    }

    /// Notify all subscribed consumers and remove dropped consumers from the list.
    fn notify_consumers(&mut self) {
        let context = &self.context;
        self.consumers.borrow_mut().retain(|cb| {
            if let Some(cb) = cb.upgrade() {
                cb(Rc::clone(context));
                true
            } else {
                false
            }
        });
    }
}

impl<T: Clone + PartialEq + 'static> Component for ContextProvider<T> {
    type Message = ();
    type Properties = ContextProviderProps<T>;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            children: props.children,
            context: Rc::new(props.context),
            consumers: RefCell::new(Vec::new()),
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

        let new_context = Rc::new(props.context);
        if self.context != new_context {
            self.context = new_context;
            self.notify_consumers();
        }

        should_render
    }

    fn view(&self) -> Html {
        html! { <>{ self.children.clone() }</> }
    }
}

fn find_context_provider_scope<T: Clone + PartialEq + 'static>(
    scope: &AnyScope,
) -> Option<Scope<ContextProvider<T>>> {
    let expected_type_id = TypeId::of::<ContextProvider<T>>();
    iter::successors(Some(scope), |scope| scope.get_parent())
        .filter(|scope| scope.get_type_id() == &expected_type_id)
        .cloned()
        .map(AnyScope::downcast::<ContextProvider<T>>)
        .next()
}

fn with_provider_component<T, F, R>(
    provider_scope: &Option<Scope<ContextProvider<T>>>,
    f: F,
) -> Option<R>
where
    T: Clone + PartialEq,
    F: FnOnce(&ContextProvider<T>) -> R,
{
    provider_scope
        .as_ref()
        .and_then(|scope| scope.get_component().map(|comp| f(&*comp)))
}

/// Hook for consuming context values in function components.
/// The context of the type passed as `T` is returned. If there is no such context in scope, `None` is returned.
/// A component which calls `use_context` will re-render when the data of the context changes.
///
/// More information about contexts and how to define and consume them can be found on [Yew Docs](https://yew.rs).
///
/// # Example
/// ```rust
/// # use yew_functional::{function_component, use_context};
/// # use yew::prelude::*;
/// # use std::rc::Rc;
///
/// # #[derive(Clone, Debug, PartialEq)]
/// # struct ThemeContext {
/// #    foreground: String,
/// #    background: String,
/// # }
/// #[function_component(ThemedButton)]
/// pub fn themed_button() -> Html {
///     let theme = use_context::<Rc<ThemeContext>>().expect("no ctx found");
///
///     html! {
///         <button style=format!("background: {}; color: {}", theme.background, theme.foreground)>
///             { "Click me" }
///         </button>
///     }
/// }
/// ```
pub fn use_context<T: Clone + PartialEq + 'static>() -> Option<Rc<T>> {
    struct UseContextState<T2: Clone + PartialEq + 'static> {
        provider_scope: Option<Scope<ContextProvider<T2>>>,
        current_context: Option<Rc<T2>>,
        callback: Option<Rc<ConsumerCallback<T2>>>,
    }
    impl<T: Clone + PartialEq + 'static> Hook for UseContextState<T> {
        fn tear_down(&mut self) {
            if let Some(cb) = self.callback.take() {
                drop(cb);
            }
        }
    }

    let scope = get_current_scope()
        .expect("No current Scope. `use_context` can only be called inside function components");

    use_hook(
        |state: &mut UseContextState<T>, hook_callback| {
            state.callback = Some(Rc::new(Box::new(move |ctx: Rc<T>| {
                hook_callback(
                    |state: &mut UseContextState<T>| {
                        state.current_context = Some(ctx);
                        true
                    },
                    false, // run pre render
                );
            })));
            let weak_cb = Rc::downgrade(state.callback.as_ref().unwrap());
            with_provider_component(&state.provider_scope, |comp| {
                comp.subscribe_consumer(weak_cb)
            });

            state.current_context.clone()
        },
        move || {
            let provider_scope = find_context_provider_scope::<T>(&scope);
            let current_context =
                with_provider_component(&provider_scope, |comp| Rc::clone(&comp.context));
            UseContextState {
                provider_scope,
                current_context,
                callback: None,
            }
        },
    )
}
