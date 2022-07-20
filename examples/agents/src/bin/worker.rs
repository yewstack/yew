use agents::native_worker::Worker;
use yew_agent::PublicAgent;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    Worker::register();
}
