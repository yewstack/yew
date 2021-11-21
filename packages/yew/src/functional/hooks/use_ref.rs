use crate::{functional::use_hook, NodeRef};
use std::{cell::RefCell, rc::Rc};

/// This hook is used for obtaining a mutable reference to a stateful value.
/// Its state persists across renders.
///
/// It is important to note that you do not get notified of state changes.
/// If you need the component to be re-rendered on state change, consider using [`use_state`](super::use_state()).
///
/// # Example
/// ```rust
/// # use yew::prelude::*;
/// # use web_sys::HtmlInputElement;
/// # use std::rc::Rc;
/// # use std::cell::RefCell;
/// # use std::ops::{Deref, DerefMut};
/// #
/// #[function_component(UseRef)]
/// fn ref_hook() -> Html {
///     let message = use_state(|| "".to_string());
///     let message_count = use_mut_ref(|| 0);
///
///     let onclick = Callback::from(move |e| {
///         let window = gloo_utils::window();
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
///           Callback::from(move |e: Event| {
///             let input: HtmlInputElement = e.target_unchecked_into();
///             message.set(input.value())
///         })
///     };
///
///     html! {
///         <div>
///             <input {onchange} value={(*message).clone()} />
///             <button {onclick}>{ "Send" }</button>
///         </div>
///     }
/// }
/// ```
pub fn use_mut_ref<T: 'static>(initial_value: impl FnOnce() -> T) -> Rc<RefCell<T>> {
    use_hook(
        || Rc::new(RefCell::new(initial_value())),
        |state, _| state.clone(),
        |_| {},
    )
}

/// This hook is used for obtaining a immutable reference to a stateful value.
/// Its state persists across renders.
///
/// If you need a mutable reference, consider using [`use_mut_ref`](super::use_mut_ref).
/// If you need the component to be re-rendered on state change, consider using [`use_state`](super::use_state()).
pub fn use_ref<T: 'static>(initial_value: impl FnOnce() -> T) -> Rc<T> {
    use_hook(
        || Rc::new(initial_value()),
        |state, _| Rc::clone(state),
        |_| {},
    )
}

/// This hook is used for obtaining a [`NodeRef`].
/// It persists across renders.
///
/// It is important to note that you do not get notified of state changes.
///
/// # Example
/// ```rust
/// # use wasm_bindgen::{prelude::Closure, JsCast};
/// # use yew::{
/// #    function_component, html, use_effect_with_deps, use_node_ref
/// # };
/// # use web_sys::{Event, HtmlElement};
///
/// #[function_component(UseNodeRef)]
/// pub fn node_ref_hook() -> Html {
///     let div_ref = use_node_ref();
///
///     {
///         let div_ref = div_ref.clone();
///         
///         use_effect_with_deps(
///             |div_ref| {
///                 let div = div_ref
///                     .cast::<HtmlElement>()
///                     .expect("div_ref not attached to div element");
///
///                 let listener = Closure::<dyn Fn(Event)>::wrap(Box::new(|_| {
///                     web_sys::console::log_1(&"Clicked!".into());
///                 }));
///
///                 div.add_event_listener_with_callback(
///                     "click",
///                     listener.as_ref().unchecked_ref(),
///                 )
///                 .unwrap();
///
///                 move || {
///                     div.remove_event_listener_with_callback(
///                         "click",
///                         listener.as_ref().unchecked_ref(),
///                     )
///                     .unwrap();
///                 }
///             },
///             div_ref,
///         );
///     }
///
///     html! {
///         <div ref={div_ref}>
///             { "Click me and watch the console log!" }
///         </div>
///     }
/// }
///
/// ```
pub fn use_node_ref() -> NodeRef {
    use_hook(NodeRef::default, |state, _| state.clone(), |_| {})
}
