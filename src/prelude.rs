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
    ComponentLink,
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

pub use agent::{
    Ambit,
    Bridge,
    Worker,
};

/// Prelude module for creating worker.
pub mod worker {
    pub use agent::{
        Addr,
        Agent,
        AgentLink,
        HandlerId,
        Transferable,
        Worker,
    };
}
