extern crate yew;
extern crate todomvc;

use yew::prelude::*;
use todomvc::Model;

fn main() {
    yew::initialize();
    let app: App<_, Model> = App::new(());
    app.mount_to_body();
    yew::run_loop();
}

