extern crate yew;
extern crate npm_and_rest;

use yew::prelude::*;
use npm_and_rest::Model;

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
