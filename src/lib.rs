#![allow(clippy::needless_doctest_main)]
//! # Yew Framework - API Documentation
//!
//! Yew is a framework for web-client apps created with
//! a modern Rust-to-Wasm compilation feature.
//! This framework was highly inspired by
//! [Elm](http://elm-lang.org/) and [React](https://reactjs.org/).
//!
//! Minimal example:
//!
//! ```rust
//! use yew::prelude::*;
//!
//! struct Model {
//!     link: ComponentLink<Self>,
//!     value: i64,
//! }
//!
//! enum Msg {
//!     DoIt,
//! }
//!
//! impl Component for Model {
//!     type Message = Msg;
//!     type Properties = ();
//!     fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
//!         Self {
//!             link,
//!             value: 0,
//!         }
//!     }
//!
//!     fn update(&mut self, msg: Self::Message) -> ShouldRender {
//!         match msg {
//!             Msg::DoIt => self.value = self.value + 1
//!         }
//!         true
//!     }
//!
//!     fn view(&self) -> Html {
//!         html! {
//!             <div>
//!                 <button onclick=self.link.callback(|_| Msg::DoIt)>{ "+1" }</button>
//!                 <p>{ self.value }</p>
//!             </div>
//!         }
//!     }
//! }
//!# fn dont_execute() {
//! fn main() {
//!     yew::initialize();
//!     App::<Model>::new().mount_to_body();
//!     yew::run_loop();
//! }
//!# }
//! ```
//!

#![deny(
    missing_docs,
    missing_debug_implementations,
    bare_trait_objects,
    anonymous_parameters,
    elided_lifetimes_in_paths
)]
#![allow(macro_expanded_macro_exports_accessed_by_absolute_paths)]
#![recursion_limit = "512"]
extern crate self as yew;

use proc_macro_hack::proc_macro_hack;
/// This macro implements JSX-like templates.
#[proc_macro_hack(support_nested)]
pub use yew_macro::html;

#[doc(hidden)]
#[proc_macro_hack(support_nested)]
pub use yew_macro::html_nested;

/// This module contains macros which implements html! macro and JSX-like templates
pub mod macros {
    pub use crate::html;
    pub use crate::html_nested;
    pub use yew_macro::Properties;
}

pub mod app;
pub mod callback;
pub mod components;
pub mod format;
pub mod html;
pub mod scheduler;
pub mod utils;
pub mod virtual_dom;

#[cfg(feature = "agent")]
pub mod agent;
#[cfg(feature = "services")]
pub mod services;

/// The module that contains all events available in the framework.
pub mod events {
    pub use crate::html::{ChangeData, InputData};

    pub use stdweb::web::event::{
        BlurEvent, ClickEvent, ContextMenuEvent, DoubleClickEvent, DragDropEvent, DragEndEvent,
        DragEnterEvent, DragEvent, DragExitEvent, DragLeaveEvent, DragOverEvent, DragStartEvent,
        FocusEvent, GotPointerCaptureEvent, IKeyboardEvent, IMouseEvent, IPointerEvent,
        KeyDownEvent, KeyPressEvent, KeyUpEvent, LostPointerCaptureEvent, MouseDownEvent,
        MouseEnterEvent, MouseLeaveEvent, MouseMoveEvent, MouseOutEvent, MouseOverEvent,
        MouseUpEvent, MouseWheelEvent, PointerCancelEvent, PointerDownEvent, PointerEnterEvent,
        PointerLeaveEvent, PointerMoveEvent, PointerOutEvent, PointerOverEvent, PointerUpEvent,
        ScrollEvent, SubmitEvent, TouchCancel, TouchEnd, TouchEnter, TouchMove, TouchStart,
    };
}

/// Initializes yew framework. It should be called first.
pub fn initialize() {
    stdweb::initialize();
}

/// Starts event loop.
pub fn run_loop() {
    stdweb::event_loop();
}

/// Starts an app mounted to a body of the document.
pub fn start_app<COMP>()
where
    COMP: Component,
    COMP::Properties: Default,
{
    initialize();
    App::<COMP>::new().mount_to_body();
    run_loop();
}

/// Starts an app mounted to a body of the document.
pub fn start_app_with_props<COMP>(props: COMP::Properties)
where
    COMP: Component,
{
    initialize();
    App::<COMP>::new().mount_to_body_with_props(props);
    run_loop();
}

/// The Yew Prelude
///
/// The purpose of this module is to alleviate imports of many common types:
///
/// ```
/// # #![allow(unused_imports)]
/// use yew::prelude::*;
/// ```
pub mod prelude {
    #[cfg(feature = "agent")]
    pub use crate::agent::{Bridge, Bridged, Threaded};
    pub use crate::app::App;
    pub use crate::callback::Callback;
    pub use crate::events::*;
    pub use crate::html::{
        Children, ChildrenWithProps, Component, ComponentLink, Href, Html, NodeRef, Properties,
        Renderable, ShouldRender,
    };
    pub use crate::macros::*;
    pub use crate::utils::NodeSeq;
    pub use crate::virtual_dom::Classes;

    /// Prelude module for creating worker.
    #[cfg(feature = "agent")]
    pub mod worker {
        pub use crate::agent::{
            Agent, AgentLink, Bridge, Bridged, Context, Global, HandlerId, Job, Private, Public,
        };
    }
}

pub use self::prelude::*;
