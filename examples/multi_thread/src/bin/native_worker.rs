
extern crate web_logger;
extern crate yew;
extern crate multi_thread;

use yew::prelude::*;
use multi_thread::native_worker;

fn main() {
    web_logger::init();
    yew::initialize();
    native_worker::Worker::register();
    yew::run_loop();
}
