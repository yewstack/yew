fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app_in_body::<multi_thread::Model>();
}
