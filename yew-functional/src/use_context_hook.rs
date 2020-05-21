// Naming this file use_context could be confusing. Not least to the IDE.
use super::{get_current_scope, use_hook, Hook};
use std::any::TypeId;
use std::cell::RefCell;
use std::iter;
use std::rc::{Rc, Weak};
use yew::html::{AnyScope, Renderable, Scope};
use yew::{html, Children, Component, ComponentLink, Html, Properties};

type ConsumerCallback<T> = Box<dyn Fn(Rc<T>)>;

#[derive(Clone, PartialEq, Properties)]
pub struct ContextProviderProps<T: Clone> {
    pub context: T,
    pub children: Children,
}

pub struct ContextProvider<T: Clone + 'static> {
    context: Rc<T>,
    children: Children,
    consumers: RefCell<Vec<Weak<ConsumerCallback<T>>>>,
}

impl<T: Clone> ContextProvider<T> {
    fn subscribe_consumer(&self, callback: Weak<ConsumerCallback<T>>) {
        // TODO perhaps we should clean the consumers vector on every subscription
        self.consumers.borrow_mut().push(callback);
    }
}

impl<T: Clone> ContextProvider<T> {
    fn notify_consumers(&mut self) {
        let context = &self.context;
        self.consumers.borrow_mut().retain(|cb| {
            if let Some(cb) = cb.upgrade() {
                cb(Rc::clone(&context));
                true
            } else {
                false
            }
        });
    }
}

impl<T: Clone + 'static> Component for ContextProvider<T> {
    type Message = ();
    type Properties = ContextProviderProps<T>;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        ContextProvider {
            children: props.children,
            context: Rc::new(props.context),
            consumers: RefCell::new(Vec::new()),
        }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        let mut should_render = false;
        if self.children != props.children {
            self.children = props.children;
            // only re-render if the children changed
            should_render = true;
        }

        self.context = Rc::new(props.context);
        self.notify_consumers();

        should_render
    }

    fn view(&self) -> Html {
        return html! {
            <>
                { self.children.render() }
            </>
        };
    }
}

fn find_context_provider_scope<T: 'static + Clone>(
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
    T: Clone,
    F: FnOnce(&ContextProvider<T>) -> R,
{
    provider_scope
        .as_ref()
        .and_then(|scope| scope.get_component().map(|comp| f(&*comp)))
}

pub fn use_context<T: 'static + Clone>() -> Option<Rc<T>> {
    let scope = get_current_scope()
        .expect("No current Scope. `use_context` can only be called inside functional components");

    struct UseContextState<T2: 'static + Clone> {
        provider_scope: Option<Scope<ContextProvider<T2>>>,
        current_context: Option<Rc<T2>>,
        callback: Option<Rc<ConsumerCallback<T2>>>,
    }
    impl<T: 'static + Clone> Hook for UseContextState<T> {
        fn tear_down(&mut self) {
            if let Some(cb) = self.callback.take() {
                drop(cb);
            }
        }
    }
    use_hook(
        |state: &mut UseContextState<T>, hook_update| {
            state.callback = Some(Rc::new(Box::new(move |ctx: Rc<T>| {
                hook_update(|state: &mut UseContextState<T>| {
                    state.current_context = Some(ctx);
                    true
                });
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
