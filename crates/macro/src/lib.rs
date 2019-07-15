use proc_macro_hack::proc_macro_hack;

/// This macro implements JSX-like templates.
#[proc_macro_hack(support_nested)]
pub use yew_macro_impl::html;
