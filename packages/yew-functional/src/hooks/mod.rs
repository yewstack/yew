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

use crate::CURRENT_HOOK;
use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;

pub trait Hook {
    fn tear_down(&mut self) {}
}

pub fn use_hook<InternalHookState, HookRunner, R, InitialStateProvider, HookUpdate: 'static>(
    hook_runner: HookRunner,
    initial_state_producer: InitialStateProvider,
) -> R
where
    HookRunner: FnOnce(&mut InternalHookState, Box<dyn Fn(HookUpdate, bool)>) -> R,
    InternalHookState: Hook + 'static,
    InitialStateProvider: FnOnce() -> InternalHookState,
    HookUpdate: FnOnce(&mut InternalHookState) -> bool,
{
    // Extract current hook
    let (hook, process_message) = CURRENT_HOOK.with(|hook_state| {
        // Determine which hook position we're at and increment for the next hook
        let hook_pos = hook_state.counter;
        hook_state.counter += 1;

        // Initialize hook if this is the first call
        if hook_pos >= hook_state.hooks.len() {
            let initial_state = Rc::new(RefCell::new(initial_state_producer()));
            hook_state.hooks.push(initial_state.clone());
            hook_state.destroy_listeners.push(Box::new(move || {
                initial_state.borrow_mut().deref_mut().tear_down();
            }));
        }

        let hook = hook_state.hooks[hook_pos].clone();

        (hook, hook_state.process_message.clone())
    });

    let hook: Rc<RefCell<InternalHookState>> = hook
        .downcast()
        .expect("Incompatible hook type. Hooks must always be called in the same order");

    let hook_callback = {
        let hook = hook.clone();
        Box::new(move |update: HookUpdate, post_render| {
            let hook = hook.clone();
            process_message(
                Box::new(move || {
                    let mut hook = hook.borrow_mut();
                    update(&mut hook)
                }),
                post_render,
            );
        })
    };

    // Execute the actual hook closure we were given. Let it mutate the hook state and let
    // it create a callback that takes the mutable hook state.
    let mut hook = hook.borrow_mut();
    hook_runner(&mut hook, hook_callback)
}
