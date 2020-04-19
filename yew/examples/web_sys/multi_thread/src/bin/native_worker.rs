use yew::agent::Threaded;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    multi_thread_web_sys::native_worker::Worker::register();
}
