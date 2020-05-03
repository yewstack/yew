use game_of_life::{Model, Msg};
use log::trace;
use yew::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    trace!("Initializing yew...");
    yew::initialize();
    App::<Model>::new()
        .mount_to_body()
        .send_message(Msg::Random);
    yew::run_loop();
}
