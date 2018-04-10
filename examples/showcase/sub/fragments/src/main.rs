extern crate yew;
extern crate fragments;

use yew::prelude::*;
use fragments::Model;

fn main() {
    yew::initialize();
    let app: App<_, Model> = App::new(());
    app.mount_to_body();
    yew::run_loop();
}
