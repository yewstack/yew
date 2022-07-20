use std::ops::Deref;
use std::rc::Rc;

use yew::prelude::*;

#[derive(Clone)]
pub struct UseBoolToggleHandle {
    value: UseStateHandle<bool>,
    toggle: Rc<dyn Fn()>,
}

impl UseBoolToggleHandle {
    pub fn toggle(self) {
        (self.toggle)()
    }
}

impl Deref for UseBoolToggleHandle {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// This hook can be used to cause a re-render with the non-default value, which is
/// then reset to the default value after that render.
///
/// # Arguments
///
/// * `default` - The default value.
///
/// # Example
/// ```
/// use crate::hooks::use_bool_toggle::use_bool_toggle;
/// ...
/// let value = use_bool_toggle(false);
/// ...
/// let onclick = {
///     let value = value.clone();
///     move |_| {
///         value.toggle();
///         // This will toggle the value to true.
///         // Then render.
///         // Post render it will toggle back to false skipping the render.
///     }
/// }
/// <button {onclick}>{ "Click me" }</button>
/// ...
/// ```
#[hook]
pub fn use_bool_toggle(default: bool) -> UseBoolToggleHandle {
    let state = use_state_eq(|| default);

    let toggle = {
        let state = state.clone();
        Rc::new(move || state.set(!*state))
    };

    UseBoolToggleHandle {
        value: state,
        toggle,
    }
}
