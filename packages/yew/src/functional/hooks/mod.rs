mod use_context;
mod use_effect;
mod use_reducer;
mod use_ref;
mod use_state;

pub use use_context::*;
pub use use_effect::*;
pub use use_reducer::*;
pub use use_ref::*;
pub use use_state::*;

use crate::functional::{HookStates, HookUpdater, CURRENT_HOOK};

/// A trait that is implemented on hooks.
///
/// A hook is usually defined via `#[hooks]`. Please refer to its documentation on how to implement
/// hooks.
pub trait Hook {
    /// The return type when a hook is run.
    type Output;

    /// Runs the hook inside current state, returns output upon completion.
    fn run(self, states: &mut HookStates) -> Self::Output;
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
///
/// See the pre-defined hooks for examples of how to use this function.
///
/// [Yew Docs]: https://yew.rs/next/concepts/function-components/custom-hooks
pub fn use_hook<InternalHook: 'static, Output, Tear: FnOnce(&mut InternalHook) + 'static>(
    initializer: impl FnOnce() -> InternalHook,
    runner: impl FnOnce(&mut InternalHook, HookUpdater) -> Output,
    destructor: Tear,
) -> Output {
    if !CURRENT_HOOK.is_set() {
        panic!("Hooks can only be used in the scope of a function component");
    }

    // Extract current hook
    let updater = CURRENT_HOOK
        .with(|hook_state| hook_state.next_state::<InternalHook, _, _>(initializer, destructor));

    // Execute the actual hook closure we were given. Let it mutate the hook state and let
    // it create a callback that takes the mutable hook state.
    let mut hook = updater.hook.borrow_mut();
    let hook: &mut InternalHook = hook
        .downcast_mut()
        .expect("Incompatible hook type. Hooks must always be called in the same order");

    runner(hook, updater.clone())
}

/// Experimental Implementation of `use_hook` based on the [`Hook`] trait.
///
/// Not efficient due to excessive boxing, but will be worked around if primitive hooks are re-implemented
/// without this hook.
pub(crate) fn use_hook_next<'hook, T, INIT, RUN, TEAR, O>(
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

        fn run(self, states: &mut HookStates) -> Self::Output {
            let Self {
                initializer,
                runner,
                destructor,
            } = self;

            // Extract current hook
            let updater = states.next_state::<T, _, _>(initializer, destructor);

            // Execute the actual hook closure we were given. Let it mutate the hook state and let
            // it create a callback that takes the mutable hook state.
            let mut hook = updater.hook.borrow_mut();
            let hook: &mut T = hook
                .downcast_mut()
                .expect("Incompatible hook type. Hooks must always be called in the same order");

            runner(hook, updater.clone())
        }
    }

    HookProvider {
        initializer: Box::new(initializer),
        runner: Box::new(runner),
        destructor: Box::new(destructor),
    }
}
