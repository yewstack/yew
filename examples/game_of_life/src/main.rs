extern crate yew;
extern crate game_of_life;

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
    yew::initialize();
    let context = Context {
        interval: IntervalService::new(),
    };
    let app: App<_, GameOfLife> = App::new(context);

    // Send initial message. For demo purposes only!
    // You should prefer to initialize everything in `Component::create` implementation.
    app.get_env().activator().send_message(Msg::Random);

    app.mount_to_body();
    yew::run_loop();
}
