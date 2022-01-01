//! This module contains Yew's web worker implementation.

mod hooks;

pub use hooks::{use_bridge, UseBridgeHandle};
#[doc(inline)]
pub use gloo_worker::*;
