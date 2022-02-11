use simple_ssr::App;
use yew::prelude::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    Renderer::<App>::new().hydrate();
}
