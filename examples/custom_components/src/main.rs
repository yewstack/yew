extern crate yew;
extern crate custom_components;

use yew::prelude::*;
use custom_components::Model;

fn main() {
    yew::initialize();
    let app: App<_, Model> = App::new(());
    app.mount_to_body();
    yew::run_loop();
}
