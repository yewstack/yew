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
    Renderable,
    ShouldRender,
};

pub use stdweb::web::event::{
    ClickEvent,
    DoubleClickEvent,
    KeyPressEvent,
    KeyDownEvent,
    KeyUpEvent,
    MouseMoveEvent,
    BlurEvent,
};

pub use app::App;

pub use callback::Callback;
