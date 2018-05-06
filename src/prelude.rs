//! The Yew Prelude
//!
//! The purpose of this module is to alleviate imports of many common types:
//!
//! ```
//! # #![allow(unused_imports)]
//! use yew::prelude::*;
//! ```

pub use html::{
    Component,
    Env,
    Href,
    Html,
    ChangeData,
    InputData,
    KeyData,
    MouseData,
    Renderable,
    ShouldRender,
};

pub use app::App;

pub use callback::Callback;
