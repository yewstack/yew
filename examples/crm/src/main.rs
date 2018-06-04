extern crate yew;
extern crate crm;

use yew::prelude::*;
use crm::Model;

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
