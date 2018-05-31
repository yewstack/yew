extern crate web_logger;
extern crate yew;
extern crate multi_thread;

use yew::prelude::*;
use multi_thread::Model;
use multi_thread::worker::Worker;

pub struct Context {
    pub worker: Addr<Worker>,
}

impl AsRef<Addr<Worker>> for Context {
    fn as_ref(&self) -> &Addr<Worker> {
        &self.worker
    }
}

fn main() {
    web_logger::init();
    match yew::initialize() {
        Ambit::Application => {
            let context = Context {
                worker: Worker::spawn(),
            };
            let app: App<_, Model> = App::new(context);
            app.mount_to_body();
            yew::run_loop();
        }
        Ambit::Agent => {
            Worker::register();
            yew::run_agent();
        }
    }
}
