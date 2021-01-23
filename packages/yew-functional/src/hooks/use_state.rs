use super::{use_hook, Hook};
use std::rc::Rc;

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
pub fn use_state<T, F>(initial_state_fn: F) -> (Rc<T>, Rc<impl Fn(T)>)
where
    F: FnOnce() -> T,
    T: 'static,
{
    struct UseStateState<T2> {
        current: Rc<T2>,
    }
    impl<T> Hook for UseStateState<T> {}
    use_hook(
        |prev: &mut UseStateState<T>, hook_callback| {
            let current = prev.current.clone();
            (
                current,
                Rc::new(move |o: T| {
                    hook_callback(
                        |state: &mut UseStateState<T>| {
                            state.current = Rc::new(o);
                            true
                        },
                        false, // run pre render
                    )
                }),
            )
        },
        move || UseStateState {
            current: Rc::new(initial_state_fn()),
        },
    )
}
