use crate::{get_current_scope, use_hook};
use std::any::TypeId;
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::{iter, mem};
use yew::html;
use yew::html::{AnyScope, Scope};
use yew::{Children, Component, ComponentLink, Html, Properties};

type ConsumerCallback<T> = Box<dyn Fn(Rc<T>)>;
type UseContextOutput<T> = Option<Rc<T>>;

struct UseContext<T2: Clone + PartialEq + 'static> {
    provider_scope: Option<Scope<ContextProvider<T2>>>,
    current_context: Option<Rc<T2>>,
    callback: Option<Rc<ConsumerCallback<T2>>>,
}

pub fn use_context<T: Clone + PartialEq + 'static>() -> UseContextOutput<T> {
    let scope = get_current_scope()
        .expect("No current Scope. `use_context` can only be called inside function components");

    use_hook(
        // Initializer
        move || {
            let provider_scope = find_context_provider_scope::<T>(&scope);
            let current_context =
                with_provider_component(&provider_scope, |comp| Rc::clone(&comp.context));

            UseContext {
                provider_scope,
                current_context,
                callback: None,
            }
        },
        // Runner
        |hook, updater| {
            // setup a listener for the context provider to update us
            let listener = move |ctx: Rc<T>| {
                updater.callback(move |state: &mut UseContext<T>| {
                    state.current_context = Some(ctx);
                    true
                });
            };
            hook.callback = Some(Rc::new(Box::new(listener)));

            // Subscribe to the context provider with our callback
            let weak_cb = Rc::downgrade(hook.callback.as_ref().unwrap());
            with_provider_component(&hook.provider_scope, |comp| {
                comp.subscribe_consumer(weak_cb)
            });

            // Return the current state
            hook.current_context.clone()
        },
        // Cleanup
        |hook| {
            if let Some(cb) = hook.callback.take() {
                drop(cb);
            }
        },
    )
}

#[derive(Clone, PartialEq, Properties)]
pub struct ContextProviderProps<T: Clone + PartialEq> {
    pub context: T,
    pub children: Children,
}

pub struct ContextProvider<T: Clone + PartialEq + 'static> {
    context: Rc<T>,
    children: Children,
    consumers: RefCell<Vec<Weak<ConsumerCallback<T>>>>,
}

impl<T: Clone + PartialEq> ContextProvider<T> {
    /// Add the callback to the subscriber list to be called whenever the context changes.
    /// The consumer is unsubscribed as soon as the callback is dropped.
    fn subscribe_consumer(&self, mut callback: Weak<ConsumerCallback<T>>) {
        // consumers re-subscribe on every render. Try to keep the subscriber list small by reusing dead slots.
        let mut consumers = self.consumers.borrow_mut();
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
