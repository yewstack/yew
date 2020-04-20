use yew::agent::Threaded;

fn main() {
    web_logger::init();
    yew::initialize();
    multi_thread::native_worker::Worker::register();
    yew::run_loop();
}
