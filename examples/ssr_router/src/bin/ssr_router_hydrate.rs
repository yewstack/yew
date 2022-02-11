use yew::prelude::*;

use function_router::App;

fn main() {
    #[cfg(target_arch = "wasm32")]
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    Renderer::<App>::new().hydrate();
}
