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
//! extern crate yew;
//! use yew::html::*;
//!
//! struct Model {
//!     value: i64,
//! }
//!
//! enum Msg {
//!     DoIt,
//! }
//!
//! fn update(context: &mut Context<Msg>, model: &mut Model, msg: Msg) {
//!     match msg {
//!         Msg::DoIt => {
//!             model.value = model.value + 1;
//!         }
//!     }
//! }
//!
//! fn view(model: &Model) -> Html<Msg> {
//!     html! {
//!         <div>
//!             <button onclick=|_| Msg::Increment,>{ "Add +1" }</button>
//!             <p>{ model.value }</p>
//!         </div>
//!     }
//! }
//!
//! fn main() {
//!     let model = Model {
//!         value: 0,
//!     };
//!     program(model, update, view);
//! }
//! ```
//!

#![deny(missing_docs)]
#![recursion_limit = "256"]

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
