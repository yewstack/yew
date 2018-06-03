extern crate yew;
extern crate counter;

use yew::prelude::*;
use counter::Model;

fn main() {
    yew::initialize();
    let app: App<_, Model> = App::new(());
    app.mount_to_body();
    yew::run_loop();
}
