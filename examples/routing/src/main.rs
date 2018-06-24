extern crate yew;
extern crate routing;

use yew::prelude::*;
use routing::Model;

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}