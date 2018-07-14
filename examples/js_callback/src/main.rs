extern crate yew;
extern crate js_callback;

use yew::prelude::*;
use js_callback::Model;

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
