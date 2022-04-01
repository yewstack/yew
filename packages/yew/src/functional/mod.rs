//! Function components are a simplified version of normal components.
//! They consist of a single function annotated with the attribute `#[function_component]`
//! that receives props and determines what should be rendered by returning [`Html`](crate::Html).
//!
//! Functions with the attribute have to return `Html` and may take a single parameter for the type of props the component should accept.
//! The parameter type needs to be a reference to a `Properties` type (ex. `props: &MyProps`).
//! If the function doesn't have any parameters the resulting component doesn't accept any props.
//!
//! Just mark the component with the attribute. The component will be named after the function.
//!
//! ```rust
//! # use yew::prelude::*;
//! #
//! #[function_component]
//! fn HelloWorld() -> Html {
//!     html! { "Hello world" }
//! }
//! ```
//!
//! More details about function components and Hooks can be found on [Yew Docs](https://yew.rs/docs/next/concepts/function-components/introduction)

use crate::html::{AnyScope, BaseComponent, HtmlResult};
use crate::Properties;
use std::any::Any;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

mod hooks;
pub use hooks::*;

use crate::html::{Context, RefProp};

use crate::html::sealed::SealedBaseComponent;

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

/// This attribute creates a user-defined hook from a normal Rust function.
pub use yew_macro::hook;

type ReRender = Rc<dyn Fn()>;

/// Primitives of a Hook state.
pub(crate) trait Effect {
    fn rendered(&self) {}
}

/// A hook context to be passed to hooks.
pub struct HookContext {
    pub(crate) scope: AnyScope,
    re_render: ReRender,

    states: Vec<Rc<dyn Any>>,
    effects: Vec<Rc<dyn Effect>>,

    counter: usize,
    #[cfg(debug_assertions)]
    total_hook_counter: Option<usize>,
}

impl HookContext {
    pub(crate) fn next_state<T>(&mut self, initializer: impl FnOnce(ReRender) -> T) -> Rc<T>
    where
        T: 'static,
    {
        // Determine which hook position we're at and increment for the next hook
        let hook_pos = self.counter;
        self.counter += 1;

        let state = match self.states.get(hook_pos).cloned() {
            Some(m) => m,
            None => {
                let initial_state = Rc::new(initializer(self.re_render.clone()));
                self.states.push(initial_state.clone());

                initial_state
            }
        };

        state.downcast().unwrap()
    }

    pub(crate) fn next_effect<T>(&mut self, initializer: impl FnOnce(ReRender) -> T) -> Rc<T>
    where
        T: 'static + Effect,
    {
        let prev_state_len = self.states.len();
        let t = self.next_state(initializer);

        // This is a new effect, we add it to effects.
        if self.states.len() != prev_state_len {
            self.effects.push(t.clone());
        }

        t
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
    type Properties: Properties + PartialEq;

    /// Render the component. This function returns the [`Html`](crate::Html) to be rendered for the component.
    ///
    /// Equivalent of [`Component::view`](crate::html::Component::view).
    fn run(ctx: &mut HookContext, props: &Self::Properties) -> HtmlResult;
}

/// Wrapper that allows a struct implementing [`FunctionProvider`] to be consumed as a component.
pub struct FunctionComponent<T>
where
    T: FunctionProvider + 'static,
{
    _never: std::marker::PhantomData<T>,
    hook_ctx: RefCell<HookContext>,
}

impl<T> fmt::Debug for FunctionComponent<T>
where
    T: FunctionProvider + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("FunctionComponent<_>")
    }
}

impl<T> BaseComponent for FunctionComponent<T>
where
    T: FunctionProvider + 'static,
{
    type Message = ();
    type Properties = T::Properties;

    fn create(ctx: &Context<Self>) -> Self {
        let scope = AnyScope::from(ctx.link().clone());

        Self {
            _never: std::marker::PhantomData::default(),
            hook_ctx: RefCell::new(HookContext {
                effects: Vec::new(),
                scope,
                re_render: {
                    let link = ctx.link().clone();
                    Rc::new(move || link.send_message(()))
                },
                states: Vec::new(),

                counter: 0,
                #[cfg(debug_assertions)]
                total_hook_counter: None,
            }),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> HtmlResult {
        let props = ctx.props();
        let mut ctx = self.hook_ctx.borrow_mut();
        ctx.counter = 0;

        #[allow(clippy::let_and_return)]
        let result = T::run(&mut *ctx, props);

        #[cfg(debug_assertions)]
        {
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
        }

        result
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        let hook_ctx = self.hook_ctx.borrow();

        for effect in hook_ctx.effects.iter() {
            effect.rendered();
        }

        #[cfg(debug_assertions)]
        ctx.props().ref_().assert_ref_set();
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {
        let mut hook_ctx = self.hook_ctx.borrow_mut();
        // We clear the effects as these are also references to states.
        hook_ctx.effects.clear();

        for state in hook_ctx.states.drain(..) {
            drop(state);
        }
    }
}

impl<T> SealedBaseComponent for FunctionComponent<T> where T: FunctionProvider + 'static {}
