use crate::functional::use_hook;
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

struct UseState<T2> {
    current: Rc<T2>,
}

/// This hook is used to mange state in a function component.
///
/// # Example
/// ```rust
/// # use yew::prelude::*;
/// # use std::rc::Rc;
/// #
/// #[function_component(UseState)]
/// fn state() -> Html {
///     let counter = use_state(|| 0);
///     let click = {
///         let counter = counter.clone();
///         Callback::from(move |_| counter.set(*counter + 1))
///     };
///
///
///     html! {
///         <div>
///             <button on:{click}>{ "Increment value" }</button>
///             <p>
///                 <b>{ "Current value: " }</b>
///                 { *counter }
///             </p>
///         </div>
///     }
/// }
/// ```
pub fn use_state<T: 'static, F: FnOnce() -> T + 'static>(initial_state_fn: F) -> UseStateHandle<T> {
    use_hook(
        // Initializer
        move || UseState {
            current: Rc::new(initial_state_fn()),
        },
        // Runner
        move |hook, updater| {
            let setter: Rc<(dyn Fn(T))> = Rc::new(move |new_val: T| {
                updater.callback(move |st: &mut UseState<T>| {
                    st.current = Rc::new(new_val);
                    true
                })
            });

            let current = hook.current.clone();
            UseStateHandle {
                value: current,
                setter,
            }
        },
        // Destructor
        |_| {},
    )
}

/// State handle for the [`use_state`] hook.
pub struct UseStateHandle<T> {
    value: Rc<T>,
    setter: Rc<dyn Fn(T)>,
}

impl<T: fmt::Debug> fmt::Debug for UseStateHandle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UseStateHandle")
            .field("value", &format!("{:?}", self.value))
            .finish()
    }
}

impl<T> UseStateHandle<T> {
    /// Updates the value
    pub fn set(&self, value: T) {
        (self.setter)(value)
    }
}

impl<T> Deref for UseStateHandle<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &(*self.value)
    }
}

impl<T> Clone for UseStateHandle<T> {
    fn clone(&self) -> Self {
        Self {
            value: Rc::clone(&self.value),
            setter: Rc::clone(&self.setter),
        }
    }
}
