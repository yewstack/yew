//! Function components are a simplified version of normal components.
//! They consist of a single function annotated with the attribute `#[function_component(_)]`
//! that receives props and determines what should be rendered by returning [`Html`](crate::Html).
//!
//! ```rust
//! # use yew::prelude::*;
//! #
//! #[function_component(HelloWorld)]
//! fn hello_world() -> Html {
//!     html! { "Hello world" }
//! }
//! ```
//!
//! More details about function components and Hooks can be found on [Yew Docs](https://yew.rs/docs/next/concepts/function-components/introduction)

use crate::html::{AnyScope, BaseComponent, HtmlResult};
use crate::Properties;
use std::cell::{RefCell, RefMut};
use std::fmt;
use std::ops::DerefMut;
use std::rc::Rc;

mod hooks;
pub use hooks::*;

use crate::html::Context;

use crate::html::SealedBaseComponent;

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
pub use yew_macro::function_component;

/// This attribute creates a hook from a normal Rust function.
pub use yew_macro::hook;

type Msg = Box<dyn FnOnce() -> bool>;
type ProcessMessage = Rc<dyn Fn(Msg, bool)>;

/// A hook context to be passed to hooks.
pub struct HookContext {
    scope: AnyScope,

    process_message: ProcessMessage,
    hooks: Vec<Rc<RefCell<dyn std::any::Any>>>,
    destroy_listeners: Vec<Box<dyn FnOnce()>>,

    counter: usize,
    total_hook_counter: Option<usize>,
}

impl HookContext {
    pub(crate) fn next_state<T, INIT, TEAR>(
        &mut self,
        initializer: INIT,
        destructor: TEAR,
    ) -> HookUpdater
    where
        T: 'static,
        INIT: FnOnce() -> T,
        TEAR: FnOnce(&mut T) + 'static,
    {
        // Determine which hook position we're at and increment for the next hook
        let hook_pos = self.counter;
        self.counter += 1;

        let hook = match self.hooks.get(hook_pos).cloned() {
            Some(m) => m,
            None => {
                let initial_state = Rc::new(RefCell::new(initializer()));
                self.hooks.push(initial_state.clone());

                {
                    let initial_state = initial_state.clone();
                    self.destroy_listeners.push(Box::new(move || {
                        destructor(initial_state.borrow_mut().deref_mut());
                    }));
                }
                initial_state
            }
        };

        HookUpdater {
            hook,
            scope: self.scope.clone(),
            process_message: self.process_message.clone(),
        }
    }

    fn scope(&self) -> &AnyScope {
        &self.scope
    }
}

impl fmt::Debug for HookContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("HookContext<_>")
    }
}

/// Trait that allows a struct to act as Function Component.
pub trait FunctionProvider {
    /// Properties for the Function Component.
    type TProps: Properties + PartialEq;

    /// Render the component. This function returns the [`Html`](crate::Html) to be rendered for the component.
    ///
    /// Equivalent of [`Component::view`](crate::html::Component::view).
    fn run(ctx: &mut HookContext, props: &Self::TProps) -> HtmlResult;
}

/// Wrapper that allows a struct implementing [`FunctionProvider`] to be consumed as a component.
pub struct FunctionComponent<T: FunctionProvider + 'static> {
    _never: std::marker::PhantomData<T>,
    hook_ctx: RefCell<HookContext>,
    message_queue: MsgQueue,
}

impl<T: FunctionProvider> fmt::Debug for FunctionComponent<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("FunctionComponent<_>")
    }
}

impl<T> BaseComponent for FunctionComponent<T>
where
    T: FunctionProvider + 'static,
{
    type Message = Box<dyn FnOnce() -> bool>;
    type Properties = T::TProps;

    fn create(ctx: &Context<Self>) -> Self {
        let scope = AnyScope::from(ctx.link().clone());
        let message_queue = MsgQueue::default();

        Self {
            _never: std::marker::PhantomData::default(),
            message_queue: message_queue.clone(),
            hook_ctx: RefCell::new(HookContext {
                counter: 0,
                scope,
                process_message: {
                    let scope = ctx.link().clone();
                    Rc::new(move |msg, post_render| {
                        if post_render {
                            message_queue.push(msg);
                        } else {
                            scope.send_message(msg);
                        }
                    })
                },
                hooks: vec![],
                destroy_listeners: vec![],
                total_hook_counter: None,
            }),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        msg()
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> HtmlResult {
        let props = ctx.props();
        let mut ctx = self.hook_ctx.borrow_mut();
        ctx.counter = 0;

        let result = T::run(&mut *ctx, props);

        // Procedural Macros can catch most conditionally called hooks at compile time, but it cannot
        // detect early return (as the return can be Err(_), Suspension).
        if result.is_err() {
            if let Some(m) = ctx.total_hook_counter {
                // Suspended Components can have less hooks called when suspended, but not more.
                if m < ctx.counter {
                    panic!("Hooks are called conditionally.");
                }
            }
        } else {
            match ctx.total_hook_counter {
                Some(m) => {
                    if m != ctx.counter {
                        panic!("Hooks are called conditionally.");
                    }
                }
                None => {
                    ctx.total_hook_counter = Some(ctx.counter);
                }
            }
        }

        result
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        for msg in self.message_queue.drain() {
            ctx.link().send_message(msg);
        }
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {
        let mut hook_ctx = self.hook_ctx.borrow_mut();
        for hook in hook_ctx.destroy_listeners.drain(..) {
            hook()
        }
    }
}

impl<T> SealedBaseComponent for FunctionComponent<T> where T: FunctionProvider + 'static {}

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
#[derive(Clone)]
pub(crate) struct HookUpdater {
    hook: Rc<RefCell<dyn std::any::Any>>,
    scope: AnyScope,
    process_message: ProcessMessage,
}

impl HookUpdater {
    /// Retrieves the hook state
    pub fn borrow_mut<T: 'static>(&self) -> RefMut<'_, T> {
        RefMut::map(self.hook.borrow_mut(), |m| {
            m.downcast_mut()
                .expect("incompatible hook type. Hooks must always be called in the same order")
        })
    }

    /// Registers a callback to be run immediately.
    pub fn callback<T: 'static, F>(&self, cb: F)
    where
        F: FnOnce(&mut T) -> bool + 'static,
    {
        let this = self.clone();

        // Update the component
        // We're calling "link.send_message", so we're not calling it post-render
        let post_render = false;
        (self.process_message)(
            Box::new(move || {
                let mut hook = this.borrow_mut();
                cb(&mut *hook)
            }),
            post_render,
        );
    }

    /// Registers a callback to be run after the render
    pub fn post_render<T: 'static, F>(&self, cb: F)
    where
        F: FnOnce(&mut T) -> bool + 'static,
    {
        let this = self.clone();

        // Update the component
        // We're calling "message_queue.push", so not calling it post-render
        let post_render = true;
        (self.process_message)(
            Box::new(move || {
                let mut hook = this.borrow_mut();
                cb(&mut *hook)
            }),
            post_render,
        );
    }

    pub fn scope(&self) -> &AnyScope {
        &self.scope
    }
}
