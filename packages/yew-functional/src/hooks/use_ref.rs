use super::{use_hook, Hook};
use std::cell::RefCell;
use std::rc::Rc;

/// This hook is used for obtaining a mutable reference to a stateful value.
/// Its state persists across renders.
///
/// It is important to note that you do not get notified of state changes.
/// If you need the component to be re-rendered on state change, consider using [`use_state`].
///
/// # Example
/// ```rust
/// # use yew_functional::{function_component, use_state, use_ref};
/// # use yew::prelude::*;
/// # use std::rc::Rc;
/// # use std::cell::RefCell;
/// # use std::ops::{Deref, DerefMut};
/// #
/// #[function_component(UseRef)]
/// fn ref_hook() -> Html {
///     let (message, set_message) = use_state(|| "".to_string());
///     let message_count = use_ref(|| 0);
///
///     let onclick = Callback::from(move |e| {
///         let window = yew::utils::window();
///
///         if *message_count.borrow_mut() > 3 {
///             window.alert_with_message("Message limit reached");
///         } else {
///             *message_count.borrow_mut() += 1;
///             window.alert_with_message("Message sent");
///         }
///     });
///
///     let onchange = Callback::from(move |e| {
///         if let ChangeData::Value(value) = e {
///             set_message(value)
///         }
///     });
///
///     html! {
///         <div>
///             <input onchange=onchange value=message />
///             <button onclick=onclick>{ "Send" }</button>
///         </div>
///     }
/// }
/// ```
pub fn use_ref<T: 'static, InitialProvider>(initial_value: InitialProvider) -> Rc<RefCell<T>>
where
    InitialProvider: FnOnce() -> T,
{
    #[derive(Clone)]
    struct UseRefState<T>(Rc<RefCell<T>>);
    impl<T> Hook for UseRefState<T> {}

    use_hook(
        |state: &mut UseRefState<T>, hook_callback| {
            // we need it to be a specific closure type, even if we never use it
            let _ignored = || hook_callback(|_| false, false);
            state.0.clone()
        },
        move || UseRefState(Rc::new(RefCell::new(initial_value()))),
    )
}
