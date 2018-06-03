extern crate yew;
extern crate crm;

use yew::prelude::*;
use crm::Model;

fn main() {
    yew::initialize();
    let app: App<_, Model> = App::new(());
    app.mount_to_body();
    yew::run_loop();
}
