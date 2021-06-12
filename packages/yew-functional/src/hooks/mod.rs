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

use crate::{HookUpdater, CURRENT_HOOK};
use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;

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
/// [Yew Docs]: https://yew.rs/docs/next/concepts/function-components/custom-hooks
pub fn use_hook<InternalHook: 'static, Output, Tear: FnOnce(&mut InternalHook) + 'static>(
    initializer: impl FnOnce() -> InternalHook,
    runner: impl FnOnce(&mut InternalHook, HookUpdater) -> Output,
    destructor: Tear,
) -> Output {
    // Extract current hook
    let updater = CURRENT_HOOK.with(|hook_state| {
        // Determine which hook position we're at and increment for the next hook
        let hook_pos = hook_state.counter;
        hook_state.counter += 1;

        // Initialize hook if this is the first call
        if hook_pos >= hook_state.hooks.len() {
            let initial_state = Rc::new(RefCell::new(initializer()));
            hook_state.hooks.push(initial_state.clone());
            hook_state.destroy_listeners.push(Box::new(move || {
                destructor(initial_state.borrow_mut().deref_mut());
            }));
        }

        let hook = hook_state
            .hooks
            .get(hook_pos)
            .expect("Not the same number of hooks. Hooks must not be called conditionally")
            .clone();

        HookUpdater {
            hook,
            process_message: hook_state.process_message.clone(),
        }
    });

    // Execute the actual hook closure we were given. Let it mutate the hook state and let
    // it create a callback that takes the mutable hook state.
    let mut hook = updater.hook.borrow_mut();
    let hook: &mut InternalHook = hook
        .downcast_mut()
        .expect("Incompatible hook type. Hooks must always be called in the same order");

    runner(hook, updater.clone())
}
