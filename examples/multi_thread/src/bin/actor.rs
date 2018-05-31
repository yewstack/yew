#[macro_use]
extern crate log;
extern crate web_logger;
extern crate yew;
extern crate multi_thread;

use yew::prelude::*;
use multi_thread::worker;

fn main() {
    web_logger::init();
    yew::initialize();
    debug!("Actor works!");
    worker::Worker::execute();
    debug!("And rebuilds!");
    yew::run_loop();
}
