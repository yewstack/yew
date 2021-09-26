#![recursion_limit = "1024"]
#![allow(clippy::large_enum_variant)]

pub mod app;

use app::Model;
use wasm_bindgen::prelude::*;
use yew::AppHandle;
use yew_agent::Threaded;

#[wasm_bindgen(start)]
pub fn start() {
    use js_sys::{global, Reflect};

    if Reflect::has(&global(), &JsValue::from_str("window")).unwrap() {
        yew::start_app::<Model>();
    } else {
        app::Worker::register();
    }
}
