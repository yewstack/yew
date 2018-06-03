extern crate yew;
extern crate dashboard;

use yew::prelude::*;
use dashboard::Model;

fn main() {
    yew::initialize();
    let app: App<(), Model<Context>> = App::new(());
    app.mount_to_body();
    yew::run_loop();
}
