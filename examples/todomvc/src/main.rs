extern crate yew;
extern crate todomvc;

use yew::prelude::*;
use todomvc::Model;

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}

