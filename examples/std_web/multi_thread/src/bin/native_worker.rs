use yew::agent::Threaded;

fn main() {
    web_logger::init();
    yew::initialize();
    multi_thread_std_web::native_worker::Worker::register();
    yew::run_loop();
}
