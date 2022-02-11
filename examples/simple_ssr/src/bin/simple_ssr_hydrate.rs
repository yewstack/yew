use simple_ssr::App;
use yew::prelude::*;

fn main() {
    #[cfg(target_arch = "wasm32")]
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    Renderer::<App>::new().hydrate();
}
