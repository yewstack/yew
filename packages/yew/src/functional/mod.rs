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
use wasm_bindgen::prelude::UnwrapThrowExt;

mod append_vec;
mod hooks;

use append_vec::AppendOnlyList;
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
pub(crate) trait Effect: ToAny {
    fn rendered(&mut self) {}
}

pub(crate) trait ToAny: Any {
    fn as_any(&mut self) -> &mut dyn Any;
}

impl<T: Effect> ToAny for T {
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

/// A hook context to be passed to hooks.
pub struct HookContext {
    pub(crate) scope: AnyScope,
    re_render: ReRender,

    states: AppendOnlyList<dyn Any>,
    effects: AppendOnlyList<dyn Effect>,
}

impl HookContext {
    fn new(scope: AnyScope, re_render: ReRender) -> RefCell<Self> {
        RefCell::new(HookContext {
            scope,
            re_render,
            states: AppendOnlyList::new(),
            effects: AppendOnlyList::new(),
        })
    }

    #[allow(clippy::mut_from_ref)]
    pub(crate) fn next_state<T>(&self, initializer: impl FnOnce() -> T) -> &mut T
    where
        T: 'static,
    {
        let wrapped_init = || Box::new(initializer()) as Box<dyn Any>;
        let state = self.states.next_state(wrapped_init);
        state.downcast_mut().unwrap_throw()
    }

    #[allow(clippy::mut_from_ref)]
    pub(crate) fn next_effect<T>(&self, initializer: impl FnOnce() -> T) -> &mut T
    where
        T: 'static + Effect,
    {
        let wrapped_init = || Box::new(initializer()) as Box<dyn Effect>;
        let effect = self.effects.next_state(wrapped_init);
        effect.as_any().downcast_mut().unwrap_throw()
    }

    #[inline(always)]
    fn prepare_run(&mut self) {
        self.states.restart();
        self.effects.restart();
    }

    fn run_effects(&mut self) {
        for effect in self.effects.get_mut().iter_mut() {
            effect.as_mut().rendered();
        }
    }

    fn drain_states(&mut self) {
        // We clear the effects as these are also references to states.
        self.effects.get_mut().clear();

        // Vec doesn't guarantee the order, this does
        for state in self.states.get_mut().drain(..) {
            drop(state);
        }
    }

    #[cfg(debug_assertions)]
    fn assert_hook_context(&mut self, render_ok: bool) {
        self.states.assert_hook_context(render_ok);
        self.effects.assert_hook_context(render_ok);
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

/// A type that interacts [`FunctionProvider`] to provide lifecycle events to be bridged to
/// [`BaseComponent`].
///
/// # Note
///
/// Function Components should not be implemented with this type directly.
///
/// Use the `#[function_component]` macro instead.
#[doc(hidden)]
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
        let mut hook_ctx = self.hook_ctx.borrow_mut();
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
