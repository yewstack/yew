mod use_callback;
mod use_context;
mod use_effect;
mod use_force_update;
mod use_memo;
mod use_prepared_state;
mod use_reducer;
mod use_ref;
mod use_state;
mod use_transitive_state;

pub use use_callback::*;
pub use use_context::*;
pub use use_effect::*;
pub use use_force_update::*;
pub use use_memo::*;
pub use use_prepared_state::*;
pub use use_reducer::*;
pub use use_ref::*;
pub use use_state::*;
pub use use_transitive_state::*;

use crate::functional::HookContext;

/// A trait that is implemented on hooks.
///
/// Hooks are defined via the [`#[hook]`](crate::functional::hook) macro. It provides rewrites to
/// hook invocations and ensures that hooks can only be called at the top-level of a function
/// component or a hook. Please refer to its documentation on how to implement hooks.
pub trait Hook {
    /// The return type when a hook is run.
    type Output;

    /// Runs the hook inside current state, returns output upon completion.
    fn run(self, ctx: &mut HookContext) -> Self::Output;
}

/// The blanket implementation of boxed hooks.
#[doc(hidden)]
#[allow(missing_debug_implementations, missing_docs)]
pub struct BoxedHook<'hook, T> {
    inner: Box<dyn 'hook + FnOnce(&mut HookContext) -> T>,
}

impl<'hook, T> BoxedHook<'hook, T> {
    #[allow(missing_docs)]
    pub fn new(inner: Box<dyn 'hook + FnOnce(&mut HookContext) -> T>) -> Self {
        Self { inner }
    }
}

impl<T> Hook for BoxedHook<'_, T> {
    type Output = T;

    fn run(self, ctx: &mut HookContext) -> Self::Output {
        (self.inner)(ctx)
    }
}
