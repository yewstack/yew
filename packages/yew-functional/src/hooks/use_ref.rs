use crate::use_hook;
use std::{cell::RefCell, rc::Rc};

/// This hook is used for obtaining a mutable reference to a stateful value.
/// Its state persists across renders.
///
/// It is important to note that you do not get notified of state changes.
/// If you need the component to be re-rendered on state change, consider using [`use_state`](super::use_state()).
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
///     let message = use_state(|| "".to_string());
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
///     let onchange = {
///         let message = message.clone();
///           Callback::from(move |e| {
///             if let ChangeData::Value(value) = e {
///                 message.set(value)
///             }
///         })
///     };
///
///     html! {
///         <div>
///             <input onchange=onchange value=(*message).clone() />
///             <button onclick=onclick>{ "Send" }</button>
///         </div>
///     }
/// }
/// ```
pub fn use_ref<T: 'static>(initial_value: impl FnOnce() -> T + 'static) -> Rc<RefCell<T>> {
    use_hook(
        || Rc::new(RefCell::new(initial_value())),
        |state, _| state.clone(),
        |_| {},
    )
}
