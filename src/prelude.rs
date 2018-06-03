//! The Yew Prelude
//!
//! The purpose of this module is to alleviate imports of many common types:
//!
//! ```
//! # #![allow(unused_imports)]
//! use yew::prelude::*;
//! ```
extern crate stdweb;

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

pub use app::App;

pub use callback::Callback;

pub use stdweb::web::event::{
    BlurEvent,
    ClickEvent,
    DoubleClickEvent,
    IKeyboardEvent,
    IMouseEvent,
    KeyDownEvent,
    KeyPressEvent,
    KeyUpEvent,
    MouseDownEvent,
    MouseMoveEvent,
    MouseOutEvent,
    MouseOverEvent,
    MouseUpEvent,
};

// TODO Split to `prelude::app` and `prelude::agent`

pub use agent::{
    Addr,
    Agent,
    AgentLink,
    Ambit,
    Message,
    Worker,
};
