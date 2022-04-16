mod use_callback;
mod use_context;
mod use_effect;
mod use_force_update;
mod use_memo;
mod use_reducer;
mod use_ref;
mod use_state;

pub use use_callback::*;
pub use use_context::*;
pub use use_effect::*;
pub use use_force_update::*;
pub use use_memo::*;
pub use use_reducer::*;
pub use use_ref::*;
pub use use_state::*;

use crate::functional::{AnyScope, HookContext};

/// A trait that is implemented on hooks.
///
/// Hooks are defined via the [`#[hook]`](crate::functional::hook) macro. It provides rewrites to hook invocations
/// and ensures that hooks can only be called at the top-level of a function component or a hook.
/// Please refer to its documentation on how to implement hooks.
pub trait Hook<'hook> {
    /// The return type when a hook is run.
    type Output;

    /// Runs the hook inside current state, returns output upon completion.
    fn run(self, ctx: &'hook HookContext) -> Self::Output;
}

/// The blanket implementation of boxed hooks.
#[doc(hidden)]
#[allow(missing_debug_implementations, missing_docs)]
pub struct BoxedHook<'hook, T> {
    inner: Box<dyn 'hook + FnOnce(&'hook HookContext) -> T>,
}

impl<'hook, T> BoxedHook<'hook, T> {
    #[allow(missing_docs)]
    pub fn new(inner: Box<dyn 'hook + FnOnce(&HookContext) -> T>) -> Self {
        Self { inner }
    }
}

impl<'boxed, 'hook: 'boxed, T> Hook<'hook> for BoxedHook<'boxed, T> {
    type Output = T;

    fn run(self, ctx: &'hook HookContext) -> Self::Output {
        (self.inner)(ctx)
    }
}

pub(crate) fn use_component_scope() -> impl for<'comp> Hook<'comp, Output = &'comp AnyScope> {
    struct HookProvider {}

    impl<'comp> Hook<'comp> for HookProvider {
        type Output = &'comp AnyScope;

        fn run(self, ctx: &'comp HookContext) -> Self::Output {
            &ctx.scope
        }
    }

    HookProvider {}
}
