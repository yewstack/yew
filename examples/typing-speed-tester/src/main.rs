mod app;
mod file_input;
mod results;
mod utils;

fn main() {
    // init logger
    wasm_logger::init(wasm_logger::Config::default());

    // render main page
    yew::Renderer::<app::App>::new().render();
}
