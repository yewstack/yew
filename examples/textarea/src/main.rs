extern crate yew;
extern crate textarea;

use yew::prelude::*;
use textarea::Model;

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
