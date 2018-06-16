extern crate yew;
extern crate timer;

use yew::prelude::*;
use timer::Model;

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
