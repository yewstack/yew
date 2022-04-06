use crate::callback::Callback;
use crate::functional::{hook, use_memo};

/// Get a immutable reference to a memoized `Callback`.
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
///     // will be rendered only once even when you click the button mutiple times.
///     let callback = use_callback(
///         move |name| format!("Hello, {}!", name),
///         ()
///     );
///
///     // It can also be used for events.
///     let oncallback = {
///         let counter = counter.clone();
///         use_callback(
///             move |_e| (),
///             counter
///         )
///     };
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
pub fn use_callback<IN, OUT, F, D>(f: F, deps: D) -> Callback<IN, OUT>
where
    IN: 'static,
    OUT: 'static,
    F: Fn(IN) -> OUT + 'static,
    D: PartialEq + 'static,
{
    (*use_memo(move |_| Callback::from(f), deps)).clone()
}
