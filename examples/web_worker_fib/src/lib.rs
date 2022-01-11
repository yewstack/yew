#![recursion_limit = "1024"]
#![allow(clippy::large_enum_variant)]

pub mod agent;
pub mod app;
use app::App;
use wasm_bindgen::prelude::*;
use yew_agent::PublicWorker;

#[wasm_bindgen(start)]
pub fn start() {
    use js_sys::{global, Reflect};

    if Reflect::has(&global(), &JsValue::from_str("window")).unwrap() {
        yew::start_app::<App>();
    } else {
        agent::Worker::register();
    }
}
