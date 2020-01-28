use yew::agent::Threaded;

fn main() {
    #[cfg(feature = "std_web")]
    web_logger::init();
    #[cfg(feature = "web_sys")]
    wasm_logger::init(wasm_logger::Config::default());
    yew::initialize();
    multi_thread::native_worker::Worker::register();
    yew::run_loop();
}
