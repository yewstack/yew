use std::cell::RefCell;
use std::rc::Rc;

use crate::functional::{hook, use_state, Hook, HookContext};
use crate::NodeRef;

struct UseMutRef<F> {
    init_fn: F,
}

impl<T: 'static, F: FnOnce() -> T> Hook for UseMutRef<F> {
    type Output = Rc<RefCell<T>>;

    fn run(self, ctx: &mut HookContext) -> Self::Output {
        ctx.next_state(|_| RefCell::new((self.init_fn)()))
    }
}

/// This hook is used for obtaining a mutable reference to a stateful value.
/// Its state persists across renders.
///
/// It is important to note that you do not get notified of state changes.
/// If you need the component to be re-rendered on state change, consider using
/// [`use_state`](super::use_state()).
///
/// # Example
/// ```rust
/// use std::cell::RefCell;
/// use std::ops::{Deref, DerefMut};
/// use std::rc::Rc;
///
/// use web_sys::HtmlInputElement;
/// use yew::prelude::*;
///
/// #[function_component(UseRef)]
/// fn ref_hook() -> Html {
///     let message = use_state(|| "".to_string());
///     let message_count = use_mut_ref(|| 0);
///
///     let onclick = Callback::from(move |e| {
///         let window = gloo::utils::window();
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
///         Callback::from(move |e: Event| {
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
pub fn use_mut_ref<T: 'static, F>(init_fn: F) -> impl Hook<Output = Rc<RefCell<T>>>
where
    F: FnOnce() -> T,
{
    UseMutRef { init_fn }
}

/// This hook is used for obtaining a [`NodeRef`].
/// It persists across renders.
///
/// The `ref` attribute can be used to attach the [`NodeRef`] to an HTML element. In callbacks,
/// you can then get the DOM `Element` that the `ref` is attached to.
///
/// # Example
///
/// ```rust
/// use wasm_bindgen::prelude::Closure;
/// use wasm_bindgen::JsCast;
/// use web_sys::{Event, HtmlElement};
/// use yew::{function_component, html, use_effect_with_deps, use_node_ref, Html};
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
/// ```
///
/// # Tip
///
/// When conditionally rendering elements you can use `NodeRef` in conjunction with
/// `use_effect_with_deps` to perform actions each time an element is rendered and just before the
/// component where the hook is used in is going to be removed from the DOM.
#[hook]
pub fn use_node_ref() -> NodeRef {
    (*use_state(NodeRef::default)).clone()
}
