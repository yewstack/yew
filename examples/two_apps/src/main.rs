extern crate stdweb;
extern crate yew;
extern crate two_apps;

use stdweb::web::{IParentNode, document};
use yew::prelude::*;
use yew::html::Scope;
use yew::scheduler::Scheduler;
use two_apps::{Model, Msg};

fn mount_app(selector: &'static str, app: App<Model>) -> Scope<Model> {
    let element = document().query_selector(selector).unwrap().unwrap();
    app.mount(element)
}

fn main() {
    yew::initialize();

    // Example how to reuse context in two scopes
    let scheduler = Scheduler::new();

    let first_app = App::reuse(&scheduler);
    let second_app = App::reuse(&scheduler);

    let mut to_first = mount_app(".first-app", first_app);
    let mut to_second = mount_app(".second-app", second_app);
    to_first.send_message(Msg::SetScope(to_second.clone()));
    to_second.send_message(Msg::SetScope(to_first.clone()));

    yew::run_loop();
}
