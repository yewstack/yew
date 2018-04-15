extern crate yew;
extern crate textarea;

use yew::prelude::*;
use textarea::Model;

fn main() {
    yew::initialize();
    let app: App<_, Model> = App::new(());
    app.mount_to_body();
    yew::run_loop();
}
