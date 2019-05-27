use proc_macro_hack::proc_macro_hack;

#[macro_use]
pub mod helpers;

pub use yew_shared::events;
pub use yew_shared::html;
pub use yew_shared::virtual_dom;

/// Generate html tree
#[proc_macro_hack(support_nested)]
pub use yew_macro_impl::html;
