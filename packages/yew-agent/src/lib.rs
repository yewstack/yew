//! This module contains Yew's web worker implementation.

mod hooks;

#[doc(inline)]
pub use gloo_worker::*;
pub use hooks::{use_bridge, UseBridgeHandle};
