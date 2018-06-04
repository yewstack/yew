extern crate yew;
extern crate large_table;

use yew::prelude::*;
use large_table::Model;

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
