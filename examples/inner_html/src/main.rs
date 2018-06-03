#![recursion_limit="512"]
extern crate stdweb;
extern crate yew;
extern crate inner_html;

use yew::prelude::*;

use inner_html::Model;

fn main() {
    yew::initialize();
    let app: App<_, Model> = App::new(());
    app.mount_to_body();
    yew::run_loop();
}
