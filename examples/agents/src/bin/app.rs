fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<agents::App>::new().render();
}
