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

#![deny(
    missing_docs,
    bare_trait_objects,
    anonymous_parameters,
    elided_lifetimes_in_paths
)]
#![recursion_limit = "512"]
extern crate self as yew;

/// Alias module for the procedural macro.
pub mod macros {
    pub use yew_macro::html;
}

pub mod components;
pub mod format;
pub mod services;
pub mod utils;

pub use yew_shared::*;

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

/// The Yew Prelude
///
/// The purpose of this module is to alleviate imports of many common types:
///
/// ```
/// # #![allow(unused_imports)]
/// use yew::prelude::*;
/// ```
pub mod prelude {
    pub use yew_macro::html;
    pub use yew_shared::prelude::*;
}

pub use self::prelude::*;
