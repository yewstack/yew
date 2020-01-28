fn main() {
    #[cfg(feature = "std_web")]
    web_logger::init();
    #[cfg(feature = "web_sys")]
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<multi_thread::Model>();
}
