//! The Yew Prelude
//!
//! The purpose of this module is to alleviate imports of many common types:
//!
//! ```
//! # #![allow(unused_imports)]
//! use yew::prelude::*;
//! ```

pub use html::{Component, Renderable, Html, Env, ShouldRender, Callback,
    InputData, KeyData, MouseData, Href};

use html::Scope;

/// Alias to `Scope`.
pub type App<CTX, COMP> = Scope<CTX, COMP>;
