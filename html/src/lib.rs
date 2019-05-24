use proc_macro_hack::proc_macro_hack;

#[macro_use]
pub mod helpers;

/// Generate html tree
#[proc_macro_hack(support_nested)]
pub use yew_html_impl::html;
