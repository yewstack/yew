fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<multi_thread_web_sys::Model>();
}
