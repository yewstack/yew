use std::ops::Deref;
use std::rc::Rc;
use yew::functional::use_hook;

pub struct UseBoolToggleHandle {
    value: bool,
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
/// <button onclick={Callback::once(move |_| {
///     value.toggle();
///     // This will toggle the value to true.
///     // Then render.
///     // Post render it will toggle back to false skipping the render.
/// })}>
/// ...
/// ```
pub fn use_bool_toggle(default: bool) -> UseBoolToggleHandle {
    use_hook(
        || default,
        move |hook, updater| {
            updater.post_render(move |state: &mut bool| {
                if *state != default {
                    *state = default;
                }
                false
            });

            let toggle = Rc::new(move || {
                updater.callback(move |st: &mut bool| {
                    *st = !*st;
                    true
                })
            });

            UseBoolToggleHandle {
                value: *hook,
                toggle,
            }
        },
        |_| {},
    )
}
