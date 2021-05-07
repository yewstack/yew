//! Utility library for the Yew frontend web framework.
//!
//! All features:
//!
//! * "neq" - NeqAssign trait
//! * "pure" - Pure components and function components.
//! * "future" - Async support for Yew Messages
//! * "fetch" - Wrapper that holds requests and responses.
//! * "mrc_irc" - Ergonomic Rc pointers.
//! * "lrc" - Linked-list Rc pointer.
//! * "history" - History tracker
//! * "store" - Global state with easy binding
// //! * "dsl" - Use functions instead of Yew's `html!` macro.

#[deprecated]
#[cfg(feature = "effect")]
pub use effect::{effect, Effect};
#[cfg(feature = "history")]
pub use history::History;
#[deprecated]
#[cfg(feature = "pure")]
pub use pure::{Pure, PureComponent};
/// Define a component with a function
///
/// # Example
///
/// ```
/// # use yew::{html, Callback, Html, MouseEvent};
///
/// #[yewtil::function_component(Button)]
/// pub fn button(
///     callback: &Callback<MouseEvent>,
///     #[prop_or_default] text: String,
/// ) -> Html {
///     html! {
///         <button onclick=callback>{ text }</button>
///     }
/// }
///
/// # fn view() -> Html {
/// // You can then use it like a normal component
/// html! { <Button callback=Callback::from(|_| println!("Clicked")) text="Click me!" /> }
/// # }
/// ```
#[deprecated]
#[cfg(feature = "pure")]
pub use yewtil_macro::function_component;

#[cfg(feature = "pure")]
mod pure;

#[cfg(any(feature = "mrc_irc", feature = "lrc"))]
pub mod ptr;

#[cfg(feature = "history")]
mod history;

#[cfg(feature = "fetch")]
pub mod fetch;

#[cfg(feature = "effect")]
mod effect;

#[cfg(feature = "store")]
pub mod store;
