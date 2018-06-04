extern crate yew;
extern crate custom_components;

use yew::prelude::*;
use custom_components::Model;

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
