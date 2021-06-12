//! Function components are a simplified version of normal components.
//! They consist of a single function annotated with the attribute `#[function_component(_)]`
//! that receives props and determines what should be rendered by returning [`Html`].
//!
//! ```rust
//! # use yew_functional::function_component;
//! # use yew::prelude::*;
//! #
//! #[function_component(HelloWorld)]
//! fn hello_world() -> Html {
//!     html! { "Hello world" }
//! }
//! ```
//!
//! More details about function components and Hooks can be found on [Yew Docs](https://yew.rs/docs/next/concepts/function-components)

use scoped_tls_hkt::scoped_thread_local;
use std::cell::RefCell;
use std::rc::Rc;
use yew::html::AnyScope;
use yew::{Component, ComponentLink, Html, Properties};

mod hooks;
pub use hooks::*;

/// This attribute creates a function component from a normal Rust function.
///
/// Functions with this attribute **must** return `Html` and can optionally take an argument for props.
/// Note that the function only receives a reference to the props.
///
/// When using this attribute you need to provide a name for the component:
/// `#[function_component(ComponentName)]`.
/// The attribute will then automatically create a [`FunctionComponent`] with the given identifier
/// which you can use like a normal component.
///
/// # Example
/// ```rust
/// # use yew_functional::function_component;
/// # use yew::prelude::*;
/// #
/// # #[derive(Properties, Clone, PartialEq)]
/// # pub struct Props {
/// #     text: String
/// # }
/// #
/// #[function_component(NameOfComponent)]
/// pub fn component(props: &Props) -> Html {
///     html! {
///         <p>{ &props.text }</p>
///     }
/// }
/// ```
pub use yew_functional_macro::function_component;

scoped_thread_local!(static mut CURRENT_HOOK: HookState);

type Msg = Box<dyn FnOnce() -> bool>;
type ProcessMessage = Rc<dyn Fn(Msg, bool)>;

struct HookState {
    counter: usize,
    scope: AnyScope,
    process_message: ProcessMessage,
    hooks: Vec<Rc<RefCell<dyn std::any::Any>>>,
    destroy_listeners: Vec<Box<dyn FnOnce()>>,
}

pub trait FunctionProvider {
    type TProps: Properties + PartialEq;
    fn run(props: &Self::TProps) -> Html;
}

pub struct FunctionComponent<T: FunctionProvider + 'static> {
    _never: std::marker::PhantomData<T>,
    props: T::TProps,
    hook_state: RefCell<HookState>,
    link: ComponentLink<Self>,
    message_queue: MsgQueue,
}

impl<T> FunctionComponent<T>
where
    T: FunctionProvider,
{
    fn with_hook_state<R>(&self, f: impl FnOnce() -> R) -> R {
        let mut hook_state = self.hook_state.borrow_mut();
        hook_state.counter = 0;
        CURRENT_HOOK.set(&mut *hook_state, f)
    }
}

impl<T: 'static> Component for FunctionComponent<T>
where
    T: FunctionProvider,
{
    type Message = Box<dyn FnOnce() -> bool>;
    type Properties = T::TProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let scope = AnyScope::from(link.clone());
        let message_queue = MsgQueue::default();

        Self {
            _never: std::marker::PhantomData::default(),
            props,
            link: link.clone(),
            message_queue: message_queue.clone(),
            hook_state: RefCell::new(HookState {
                counter: 0,
                scope,
                process_message: Rc::new(move |msg, post_render| {
                    if post_render {
                        message_queue.push(msg);
                    } else {
                        link.send_message(msg);
                    }
                }),
                hooks: vec![],
                destroy_listeners: vec![],
            }),
        }
    }

    fn rendered(&mut self, _first_render: bool) {
        for msg in self.message_queue.drain() {
            self.link.send_message(msg);
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        msg()
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        let mut props = props;
        std::mem::swap(&mut self.props, &mut props);
        props != self.props
    }

    fn view(&self) -> Html {
        self.with_hook_state(|| T::run(&self.props))
    }

    fn destroy(&mut self) {
        let mut hook_state = self.hook_state.borrow_mut();
        for hook in hook_state.destroy_listeners.drain(..) {
            hook()
        }
    }
}

pub(crate) fn get_current_scope() -> Option<AnyScope> {
    if CURRENT_HOOK.is_set() {
        Some(CURRENT_HOOK.with(|state| state.scope.clone()))
    } else {
        None
    }
}

#[derive(Clone, Default)]
struct MsgQueue(Rc<RefCell<Vec<Msg>>>);

impl MsgQueue {
    fn push(&self, msg: Msg) {
        self.0.borrow_mut().push(msg);
    }

    fn drain(&self) -> Vec<Msg> {
        self.0.borrow_mut().drain(..).collect()
    }
}

/// The `HookUpdater` provides a convenient interface for hooking into the lifecycle of
/// the underlying Yew Component that backs the function component.
///
/// Two interfaces are provided - callback and post_render.
/// - `callback` allows the creation of regular yew callbacks on the host component.
/// - `post_render` allows the creation of events that happen after a render is complete.
///
/// See [`use_effect`](hooks::use_effect()) and [`use_context`](hooks::use_context())
/// for more details on how to use the hook updater to provide function components
/// the necessary callbacks to update the underlying state.
#[derive(Clone)]
pub struct HookUpdater {
    hook: Rc<RefCell<dyn std::any::Any>>,
    process_message: ProcessMessage,
}
impl HookUpdater {
    pub fn callback<T: 'static, F>(&self, cb: F)
    where
        F: FnOnce(&mut T) -> bool + 'static,
    {
        let internal_hook_state = self.hook.clone();
        let process_message = self.process_message.clone();

        // Update the component
        // We're calling "link.send_message", so we're not calling it post-render
        let post_render = false;
        process_message(
            Box::new(move || {
                let mut r = internal_hook_state.borrow_mut();
                let hook: &mut T = r
                    .downcast_mut()
                    .expect("internal error: hook downcasted to wrong type");
                cb(hook)
            }),
            post_render,
        );
    }

    pub fn post_render<T: 'static, F>(&self, cb: F)
    where
        F: FnOnce(&mut T) -> bool + 'static,
    {
        let internal_hook_state = self.hook.clone();
        let process_message = self.process_message.clone();

        // Update the component
        // We're calling "messag_equeue.push", so not calling it post-render
        let post_render = true;
        process_message(
            Box::new(move || {
                let mut hook = internal_hook_state.borrow_mut();
                let hook: &mut T = hook
                    .downcast_mut()
                    .expect("internal error: hook downcasted to wrong type");
                cb(hook)
            }),
            post_render,
        );
    }
}
