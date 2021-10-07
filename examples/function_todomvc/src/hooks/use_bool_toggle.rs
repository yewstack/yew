use std::ops::Deref;
use std::rc::Rc;
use yew::functional::use_hook;

pub struct UseBoolToggleHandle {
    pub value: bool,
    pub toggle: Rc<dyn Fn()>,
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

/// Hook for toggling a boolean value.
/// This hook does not cause a re-render if the value is not the default value.
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
