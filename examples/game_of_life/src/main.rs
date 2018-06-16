extern crate yew;
extern crate game_of_life;

#[macro_use] extern crate log;
extern crate web_logger;

use yew::prelude::*;
use game_of_life::{Model, Msg};

fn main() {
    web_logger::init();
    trace!("Initializing yew...");
    yew::initialize();
    App::<Model>::new().mount_to_body()
        .send_message(Msg::Random);
    yew::run_loop();
}
