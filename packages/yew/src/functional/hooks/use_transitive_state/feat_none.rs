//! The noop variant. This is used for client side rendering when hydration is disabled.

#[doc(hidden)]
pub use crate::functional::hooks::use_prepared_state::feat_none::use_prepared_state as use_transitive_state;
