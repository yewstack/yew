extern crate yew;
extern crate fragments;

use yew::prelude::*;
use fragments::Model;

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
