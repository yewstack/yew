extern crate yew;
extern crate timer;

use yew::prelude::*;
use yew::services::timeout::TimeoutService;
use yew::services::interval::IntervalService;
use yew::services::console::ConsoleService;
use timer::Model;

struct Context {
    interval: IntervalService,
    timeout: TimeoutService,
    console: ConsoleService,
}

impl AsMut<IntervalService> for Context {
    fn as_mut(&mut self) -> &mut IntervalService {
        &mut self.interval
    }
}

impl AsMut<TimeoutService> for Context {
    fn as_mut(&mut self) -> &mut TimeoutService {
        &mut self.timeout
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
        timeout: TimeoutService::new(),
        console: ConsoleService,
    };
    let app: App<_, Model> = App::new(context);
    app.mount_to_body();
    yew::run_loop();
}
