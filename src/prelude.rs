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
    ContextMenuEvent,
    DoubleClickEvent,
    DragDropEvent,
    DragEndEvent,
    DragEnterEvent,
    DragEvent,
    DragExitEvent,
    DragLeaveEvent,
    DragOverEvent,
    DragStartEvent,
    FocusEvent,
    IKeyboardEvent,
    IMouseEvent,
    KeyDownEvent,
    KeyPressEvent,
    KeyUpEvent,
    MouseDownEvent,
    MouseMoveEvent,
    MouseOutEvent,
    MouseEnterEvent,
    MouseLeaveEvent,
    MouseOverEvent,
    MouseUpEvent,
    MouseWheelEvent,
    PointerUpEvent,
    ScrollEvent,
    SubmitEvent
};

pub use agent::{
    Bridge,
    Bridged,
    Threaded,
};

/// Prelude module for creating worker.
pub mod worker {
    pub use agent::{
        Agent,
        AgentLink,
        Bridge,
        Bridged,
        Context,
        Global,
        HandlerId,
        Job,
        Private,
        Public,
        Transferable,
    };
}
