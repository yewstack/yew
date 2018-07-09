extern crate web_logger;
extern crate yew;
extern crate multi_thread;

use yew::prelude::*;
use multi_thread::thread_worker;

fn main() {
    web_logger::init();
    yew::initialize();
    thread_worker::Worker::register();
    yew::run_loop();
}
