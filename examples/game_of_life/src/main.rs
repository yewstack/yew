extern crate yew;
extern crate game_of_life;

#[macro_use] extern crate log;
extern crate web_logger;

use yew::prelude::*;
use yew::services::interval::IntervalService;
use game_of_life::{GameOfLife, Msg};



struct Context {
    interval: IntervalService,
}

impl AsMut<IntervalService> for Context {
    fn as_mut(&mut self) -> &mut IntervalService {
        &mut self.interval
    }
}

fn main() {
    web_logger::init();

    trace!("Initializing yew...");
    yew::initialize();

    trace!("Creating a context...");
    let context = Context {
        interval: IntervalService::new(),
    };
    let app: App<_, GameOfLife> = App::new(context);

    let mut env = app.mount_to_body();
    env.send_message(Msg::Random);

    yew::run_loop();
}
