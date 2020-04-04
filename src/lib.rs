#![allow(clippy::needless_doctest_main)]
#![doc(html_logo_url = "https://static.yew.rs/logo.svg")]

//! # Yew Framework - API Documentation
//!
//! Yew is a modern Rust framework for creating multi-threaded front-end web apps with WebAssembly
//!
//! - Features a macro for declaring interactive HTML with Rust expressions. Developers who have experience using JSX in React should feel quite at home when using Yew.
//! - Achieves high performance by minimizing DOM API calls for each page render and by making it easy to offload processing to background web workers.
//! - Supports JavaScript interoperability, allowing developers to leverage NPM packages and integrate with existing JavaScript applications.
//!
//! ### Supported Targets
//! - `wasm32-unknown-unknown`
//! - `wasm32-unknown-emscripten` - (`feature = "std_web"` is required)
//! - `asmjs-unknown-emscripten` - (`feature = "std_web"` is required)
//!
//! ### Important Notes
//! - Yew is not (yet) production ready but is great for side projects and internal tools
//! - Yew supports both `web-sys` and `stdweb`, developers *must* choose one or the other using the features `"web_sys"` and `"std_web"`.
//! - Building with `cargo-web` is not supported for `web-sys`
//! - Docs.rs docs are built by default with the `"web_sys"` feature, for `"std_web"` docs, visit [`yew-stdweb`](https://docs.rs/yew-stdweb)
//!
//! ## Example
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
//!     AddOne,
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
//!             Msg::AddOne => self.value += 1
//!         }
//!         true
//!     }
//!
//!     fn change(&mut self, _: Self::Properties) -> ShouldRender {
//!         false
//!     }
//!
//!     fn view(&self) -> Html {
//!         html! {
//!             <div>
//!                 <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
//!                 <p>{ self.value }</p>
//!             </div>
//!         }
//!     }
//! }
//!
//!# fn dont_execute() {
//! fn main() {
//!     yew::initialize();
//!     App::<Model>::new().mount_to_body();
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
mod scheduler;
pub mod utils;
pub mod virtual_dom;

#[cfg(feature = "agent")]
pub mod agent;
#[cfg(feature = "services")]
pub mod services;

/// The module that contains all events available in the framework.
pub mod events {
    use cfg_if::cfg_if;

    pub use crate::html::{ChangeData, InputData};

    cfg_if! {
        if #[cfg(feature = "std_web")] {
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
        } else if #[cfg(feature = "web_sys")] {
            pub use web_sys::{
                DragEvent, Event, FocusEvent, KeyboardEvent, MouseEvent, PointerEvent, TouchEvent, UiEvent,
                WheelEvent,
            };
        }
    }
}

use cfg_match::cfg_match;

/// Initializes yew framework. It should be called first.
pub fn initialize() {
    cfg_match! {
        feature = "std_web" => stdweb::initialize(),
        feature = "web_sys" => std::panic::set_hook(Box::new(console_error_panic_hook::hook)),
    };
}

/// Starts event loop.
pub fn run_loop() {
    #[cfg(feature = "std_web")]
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
