extern crate yew;
extern crate game_of_life;

use yew::prelude::*;
use yew::services::{
    interval::IntervalService,
    console::ConsoleService,
};
use game_of_life::{GameOfLife, Msg};

struct Context {
    interval: IntervalService,
    console: ConsoleService,
}

impl AsMut<IntervalService> for Context {
    fn as_mut(&mut self) -> &mut IntervalService {
        &mut self.interval
    }
}

impl AsMut<ConsoleService> for Context {
    fn as_mut(&mut self) -> &mut ConsoleService {
        &mut self.console
    }
}

fn main() {
    yew::initialize();
    let context = Context {
        interval: IntervalService::new(),
        console: ConsoleService::new(),
    };
    let app: App<_, GameOfLife> = App::new(context);

    let mut env = app.mount_to_body();
    env.send_message(Msg::Random);

    yew::run_loop();
}
