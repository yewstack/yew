extern crate stdweb;
extern crate yew;
extern crate two_apps;

use stdweb::web::{IParentNode, document};
use yew::prelude::*;
use yew::html::Activator;
use yew::scheduler::Scheduler;
use two_apps::{Context, Model, Msg};

fn mount_app(selector: &'static str, app: App<Context, Model>) -> Activator<Context, Model> {
    let element = document().query_selector(selector).unwrap().unwrap();
    app.mount(element)
}

fn main() {
    yew::initialize();

    let context = Context::new();

    // Example how to reuse context in two scopes
    let scheduler = Scheduler::new(context);

    let first_app = App::reuse(&scheduler);
    let second_app = App::reuse(&scheduler);

    let mut to_first = mount_app(".first-app", first_app);
    let mut to_second = mount_app(".second-app", second_app);
    to_first.send_message(Msg::SetActivator(to_second.clone()));
    to_second.send_message(Msg::SetActivator(to_first.clone()));

    yew::run_loop();
}
