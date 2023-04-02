use std::rc::Rc;

use crate::callback::Callback;
use crate::functional::{hook, use_memo};

/// Get a immutable reference to a memoized `Callback`. Its state persists across renders.
/// It will be recreated only if any of the dependencies changes value.
///
/// Memoization means it will only get recreated when provided dependencies update/change.
/// This is useful when passing callbacks to optimized child components that rely on
/// PartialEq to prevent unnecessary renders.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// #[derive(Properties, PartialEq)]
/// pub struct Props {
///     pub callback: Callback<String, String>,
/// }
///
/// #[function_component(MyComponent)]
/// fn my_component(props: &Props) -> Html {
///     let greeting = props.callback.emit("Yew".to_string());
///
///     html! {
///         <>{ &greeting }</>
///     }
/// }
///
/// #[function_component(UseCallback)]
/// fn callback() -> Html {
///     let counter = use_state(|| 0);
///     let onclick = {
///         let counter = counter.clone();
///         Callback::from(move |_| counter.set(*counter + 1))
///     };
///
///     // This callback depends on (), so it's created only once, then MyComponent
///     // will be rendered only once even when you click the button multiple times.
///     let callback = use_callback((), move |name, _| format!("Hello, {}!", name));
///
///     // It can also be used for events, this callback depends on `counter`.
///     let oncallback = use_callback(counter.clone(), move |_e, counter| {
///         let _ = **counter;
///     });
///
///     html! {
///         <div>
///             <button {onclick}>{ "Increment value" }</button>
///             <button onclick={oncallback}>{ "Callback" }</button>
///             <p>
///                 <b>{ "Current value: " }</b>
///                 { *counter }
///             </p>
///             <MyComponent {callback} />
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_callback<IN, OUT, F, D>(deps: D, f: F) -> Callback<IN, OUT>
where
    IN: 'static,
    OUT: 'static,
    F: Fn(IN, &D) -> OUT + 'static,
    D: PartialEq + 'static,
{
    let deps = Rc::new(deps);

    (*use_memo(deps, move |deps| {
        let deps = deps.clone();
        let f = move |value: IN| f(value, deps.as_ref());
        Callback::from(f)
    }))
    .clone()
}
