use log::trace;
use yew::prelude::*;
use game_of_life::{Model, Msg};

fn main() {
    web_logger::init();
    trace!("Initializing yew...");
    yew::initialize();
    App::<game_of_life::Model>::new().mount_to_body()
        .send_message(Msg::Random);
    yew::run_loop();
}
