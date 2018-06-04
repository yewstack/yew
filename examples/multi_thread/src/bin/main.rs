extern crate web_logger;
extern crate yew;
extern crate multi_thread;

use yew::prelude::*;
use multi_thread::Model;
use multi_thread::worker;

fn main() {
    web_logger::init();
    match yew::initialize() {
        Ambit::Application => {
            App::<Model>::new().mount_to_body();
            yew::run_loop();
        }
        Ambit::Agent => {
            worker::Worker::register();
            yew::run_agent();
        }
    }
}
