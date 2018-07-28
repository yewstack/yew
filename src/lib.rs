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

#![deny(missing_docs)]
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
pub mod prelude;
pub mod services;
pub mod virtual_dom;
pub mod callback;
pub mod scheduler;
pub mod agent;

use std::rc::Rc;
use std::cell::RefCell;

type Shared<T> = Rc<RefCell<T>>;

struct Hidden;

/// Initializes yew framework. It should be called first.
pub fn initialize() {
    stdweb::initialize();
}

/// Starts event loop.
pub fn run_loop() {
    stdweb::event_loop();
}
