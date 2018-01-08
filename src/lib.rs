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

#![deny(
    missing_docs,
)]
#![recursion_limit="256"]

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate stdweb;

#[macro_use]
pub mod macros;
pub mod html;
pub mod services;
pub mod format;
pub mod component;
pub mod virtual_dom;

/// Initializes yew framework. It should be called first.
/// No it actually initializes `stdweb` dependency only, but later it could
/// contain own initialization code.
pub fn initialize() {
    stdweb::initialize();
}

/// Starts event loop.
pub fn run_loop() {
    stdweb::event_loop();
}
