use yew::agent::Threaded;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    multi_thread::native_worker::Worker::register();
}
