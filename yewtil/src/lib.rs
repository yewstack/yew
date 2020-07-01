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

#[cfg(feature = "neq")]
mod not_equal_assign;

#[cfg(feature = "pure")]
mod pure;

#[cfg(any(feature = "mrc_irc", feature = "lrc"))]
pub mod ptr;

#[cfg(feature = "history")]
mod history;

#[cfg(feature = "history")]
pub use history::History;

#[cfg(feature = "neq")]
pub use not_equal_assign::NeqAssign;

#[deprecated]
#[cfg(feature = "pure")]
pub use pure::{Pure, PureComponent};

#[deprecated]
#[cfg(feature = "pure")]
pub use yewtil_macro::function_component;

#[cfg(feature = "fetch")]
pub mod fetch;

#[cfg(feature = "effect")]
mod effect;
#[deprecated]
#[cfg(feature = "effect")]
pub use effect::{effect, Effect};

#[cfg(feature = "future")]
pub mod future;

#[cfg(feature = "store")]
pub mod store;
