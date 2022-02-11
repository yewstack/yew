use yew::prelude::*;

use function_router::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    Renderer::<App>::new().hydrate();
}
