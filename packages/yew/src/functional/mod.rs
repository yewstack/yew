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

use crate::html::{AnyScope, BaseComponent, Context, HtmlResult};
use crate::Properties;
use std::any::Any;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use wasm_bindgen::prelude::*;

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
    fn new(scope: AnyScope, re_render: ReRender) -> RefCell<Self> {
        RefCell::new(HookContext {
            effects: Vec::new(),
            scope,
            re_render,
            states: Vec::new(),

            counter: 0,
            #[cfg(debug_assertions)]
            total_hook_counter: None,
        })
    }

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

        state.downcast().unwrap_throw()
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

    #[inline(always)]
    fn prepare_run(&mut self) {
        self.counter = 0;
    }

    /// asserts hook counter.
    ///
    /// This function asserts that the number of hooks matches for every render.
    #[cfg(debug_assertions)]
    fn assert_hook_context(&mut self, render_ok: bool) {
        // Procedural Macros can catch most conditionally called hooks at compile time, but it cannot
        // detect early return (as the return can be Err(_), Suspension).
        match (render_ok, self.total_hook_counter) {
            // First rendered,
            // we store the hook counter.
            (true, None) => {
                self.total_hook_counter = Some(self.counter);
            }
            // Component is suspended before it's first rendered.
            // We don't have a total count to compare with.
            (false, None) => {}

            // Subsequent render,
            // we compare stored total count and current render count.
            (true, Some(total_hook_counter)) => assert_eq!(
                total_hook_counter, self.counter,
                "Hooks are called conditionally."
            ),

            // Subsequent suspension,
            // components can have less hooks called when suspended, but not more.
            (false, Some(total_hook_counter)) => assert!(
                self.counter <= total_hook_counter,
                "Hooks are called conditionally."
            ),
        }
    }

    fn run_effects(&self) {
        for effect in self.effects.iter() {
            effect.rendered();
        }
    }

    fn drain_states(&mut self) {
        // We clear the effects as these are also references to states.
        self.effects.clear();

        for state in self.states.drain(..) {
            drop(state);
        }
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
    T: FunctionProvider,
{
    _never: std::marker::PhantomData<T>,
    hook_ctx: RefCell<HookContext>,
}

impl<T> FunctionComponent<T>
where
    T: FunctionProvider + 'static,
{
    /// Creates a new function component.
    pub fn new(ctx: &Context<T>) -> Self
    where
        T: BaseComponent<Message = ()> + FunctionProvider + 'static,
    {
        let scope = AnyScope::from(ctx.link().clone());
        let re_render = {
            let link = ctx.link().clone();

            Rc::new(move || link.send_message(()))
        };

        Self {
            _never: std::marker::PhantomData::default(),
            hook_ctx: HookContext::new(scope, re_render),
        }
    }

    /// Renders a function component.
    pub fn render(&self, props: &T::Properties) -> HtmlResult {
        let mut hook_ctx = self.hook_ctx.borrow_mut();

        hook_ctx.prepare_run();

        #[allow(clippy::let_and_return)]
        let result = T::run(&mut *hook_ctx, props);

        #[cfg(debug_assertions)]
        hook_ctx.assert_hook_context(result.is_ok());

        result
    }

    /// Run Effects of a function component.
    pub fn rendered(&self) {
        let hook_ctx = self.hook_ctx.borrow();
        hook_ctx.run_effects();
    }

    /// Destroys the function component.
    pub fn destroy(&self) {
        let mut hook_ctx = self.hook_ctx.borrow_mut();
        hook_ctx.drain_states();
    }
}

impl<T> fmt::Debug for FunctionComponent<T>
where
    T: FunctionProvider + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("FunctionComponent<_>")
    }
}
