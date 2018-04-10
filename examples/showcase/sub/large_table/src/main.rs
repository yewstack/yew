extern crate yew;
extern crate large_table;

use yew::prelude::*;
use large_table::Model;

fn main() {
    yew::initialize();
    let app: App<_, Model> = App::new(());
    app.mount_to_body();
    yew::run_loop();
}
