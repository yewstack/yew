extern crate yew;
extern crate counter;

use yew::prelude::*;
use counter::Model;

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
