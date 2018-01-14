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

extern crate http;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate stdweb;

#[macro_use]
pub mod macros;
pub mod html;
pub mod prelude;
pub mod services;
pub mod format;
pub mod virtual_dom;

/// Initializes yew framework. It should be called first.
pub fn initialize() {
    stdweb::initialize();
    js! {
        var task = null;
        var pool = [];
        var routine = function() { };
        var schedule_routine = function() {
            if (task == null) {
                task = setTimeout(routine);
            }
        };
        routine = function() {
            task = null;
            // Don't process more than 25 loops per routine call
            // to keep UI responsive
            var limit = 25;
            var callback = pool.pop();
            while (callback !== undefined) {
                callback();
                limit = limit - 1;
                if (limit > 0) {
                    callback = pool.pop();
                } else {
                    break;
                }
            }
            if (pool.length > 0) {
                schedule_routine();
            }
        };
        var schedule = function(callback) {
            pool.push(callback);
            schedule_routine();
        };
        window._yew_schedule_ = schedule;
    }
}

/// Starts event loop.
pub fn run_loop() {
    stdweb::event_loop();
}
