use wasm_bindgen::prelude::*;

mod app;
mod display;
mod input;

#[wasm_bindgen]
pub fn run_app() {
    yew::start_app::<app::App>();
}
