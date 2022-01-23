mod use_context;
mod use_effect;
mod use_memo;
mod use_reducer;
mod use_ref;
mod use_state;

pub use use_context::*;
pub use use_effect::*;
pub use use_memo::*;
pub use use_reducer::*;
pub use use_ref::*;
pub use use_state::*;

use crate::functional::{hook, AnyScope, HookContext, HookUpdater};

/// A trait that is implemented on hooks.
///
/// A hook is usually defined via `#[hooks]`. Please refer to its documentation on how to implement
/// hooks.
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

/// Low level building block of creating hooks.
///
/// It is used to created the pre-defined primitive hooks.
/// Generally, it isn't needed to create hooks and should be avoided as most custom hooks can be
/// created by combining other hooks as described in [Yew Docs].
///
/// The `initializer` callback is called once to create the initial state of the hook.
/// `runner` callback handles the logic of the hook. It is called when the hook function is called.
/// `destructor`, as the name implies, is called to cleanup the leftovers of the hook.
pub(crate) fn use_hook<'hook, T, INIT, RUN, TEAR, O>(
    initializer: INIT,
    runner: RUN,
    destructor: TEAR,
) -> impl 'hook + Hook<Output = O>
where
    T: 'static,
    O: 'hook,
    INIT: 'hook + FnOnce() -> T,
    RUN: 'hook + FnOnce(&mut T, HookUpdater) -> O,
    TEAR: 'static + FnOnce(&mut T),
{
    struct HookProvider<'a, T, O> {
        initializer: Box<dyn FnOnce() -> T + 'a>,
        runner: Box<dyn FnOnce(&mut T, HookUpdater) -> O + 'a>,
        destructor: Box<dyn FnOnce(&mut T)>,
    }

    impl<T, O> Hook for HookProvider<'_, T, O>
    where
        T: 'static,
    {
        type Output = O;

        fn run(self, ctx: &mut HookContext) -> Self::Output {
            let Self {
                initializer,
                runner,
                destructor,
            } = self;

            // Extract current hook
            let updater = ctx.next_state::<T, _, _>(initializer, destructor);

            // Execute the actual hook closure we were given. Let it mutate the hook state and let
            // it create a callback that takes the mutable hook state.
            let mut hook = updater.borrow_mut::<T>();

            runner(&mut *hook, updater.clone())
        }
    }

    HookProvider {
        initializer: Box::new(initializer),
        runner: Box::new(runner),
        destructor: Box::new(destructor),
    }
}

#[hook]
pub(crate) fn use_component_scope() -> AnyScope {
    use_hook(|| (), |_, updater| updater.scope().clone(), |_| {})
}
