extern crate yew;
extern crate dashboard;

use yew::prelude::*;
use dashboard::Model;

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
