//! The Yew Prelude
//!
//! The purpose of this module is to alleviate imports of many common types:
//!
//! ```
//! # #![allow(unused_imports)]
//! use yew::prelude::*;
//! ```

pub use html::{Component, Html, ScopeRef, ShouldRender, Callback,
    InputData, KeyData, MouseData, Href};
pub use app::{App, AppContext, AppHtml};
