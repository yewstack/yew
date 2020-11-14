use crate::use_hook;
use std::rc::Rc;

struct UseState<T2> {
    current: Rc<T2>,
}

/// This hook is used to mange state in a function component.
///
/// # Example
/// ```rust
/// # use yew_functional::{function_component, use_state, use_ref};
/// # use yew::prelude::*;
/// # use std::rc::Rc;
/// #
/// #[function_component(UseState)]
/// fn state() -> Html {
///     let (
///         counter, // the returned state
///         set_counter // setter to update the state
///     ) = use_state(|| 0);
///     let onclick = {
///         let counter = Rc::clone(&counter);
///         Callback::from(move |_| set_counter(*counter + 1))
///     };
///
///     html! {
///         <div>
///             <button onclick=onclick>{ "Increment value" }</button>
///             <p>
///                 <b>{ "Current value: " }</b>
///                 { counter }
///             </p>
///         </div>
///     }
/// }
/// ```
pub fn use_state<T: 'static, F: FnOnce() -> T + 'static>(
    initial_state_fn: F,
) -> (Rc<T>, Rc<dyn Fn(T)>) {
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
            (current, setter)
        },
        // Teardown
        |_| {},
    )
}
