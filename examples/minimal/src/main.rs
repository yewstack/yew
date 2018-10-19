extern crate yew;
extern crate minimal;

use yew::prelude::*;
use minimal::Model;

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
