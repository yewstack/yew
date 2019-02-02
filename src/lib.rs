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
//! #[macro_use]
//! extern crate yew;
//! use yew::prelude::*;
//!
//! struct Model {
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
//!     fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
//!         Self {
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
//! }
//!
//! impl Renderable<Model> for Model {
//!     fn view(&self) -> Html<Self> {
//!         html! {
//!             <div>
//!                <button onclick=|_| Msg::DoIt,>{ "+1" }</button>
//!                 <p>{ self.value }</p>
//!             </div>
//!         }
//!     }
//! }
//!
//! fn main() {
//!     yew::initialize();
//!     App::<Model>::new().mount_to_body();
//!     yew::run_loop();
//! }
//! ```
//!

#![deny(missing_docs, bare_trait_objects, anonymous_parameters, elided_lifetimes_in_paths)]
#![recursion_limit = "512"]

#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
extern crate http;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate bincode;
extern crate anymap;
extern crate slab;
#[macro_use]
extern crate stdweb;
#[cfg(feature = "toml")]
extern crate toml;
#[cfg(feature = "yaml")]
extern crate serde_yaml;
#[cfg(feature = "msgpack")]
extern crate rmp_serde;
#[cfg(feature = "cbor")]
extern crate serde_cbor;

#[macro_use]
pub mod macros;
pub mod format;
pub mod html;
pub mod app;
pub mod services;
pub mod virtual_dom;
pub mod callback;
pub mod scheduler;
pub mod agent;
pub mod components;

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
    COMP: Component + Renderable<COMP>,
{
    initialize();
    App::<COMP>::new().mount_to_body();
    run_loop();
}

/// The module that contains all events available in the framework.
pub mod events {
    pub use html::{
        ChangeData,
        InputData,
    };

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
        GotPointerCaptureEvent,
        IKeyboardEvent,
        IMouseEvent,
        IPointerEvent,
        KeyDownEvent,
        KeyPressEvent,
        KeyUpEvent,
        LostPointerCaptureEvent,
        MouseDownEvent,
        MouseMoveEvent,
        MouseOutEvent,
        MouseEnterEvent,
        MouseLeaveEvent,
        MouseOverEvent,
        MouseUpEvent,
        MouseWheelEvent,
        PointerCancelEvent,
        PointerDownEvent,
        PointerEnterEvent,
        PointerLeaveEvent,
        PointerMoveEvent,
        PointerOutEvent,
        PointerOverEvent,
        PointerUpEvent,
        ScrollEvent,
        SubmitEvent
    };
}

/// The Yew Prelude
///
/// The purpose of this module is to alleviate imports of many common types:
///
/// ```
/// # #![allow(unused_imports)]
/// use yew::prelude::*;
/// ```
pub mod prelude_1 {
    pub use html::{
        Component,
        ComponentLink,
        Href,
        Html,
        Renderable,
        ShouldRender,
    };

    pub use app::App;

    pub use callback::Callback;

    pub use agent::{
        Bridge,
        Bridged,
        Threaded,
    };

    pub use events::*;

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
}

pub use self::prelude_1::*;
