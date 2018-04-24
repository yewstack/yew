extern crate stdweb;
extern crate yew;
extern crate two_apps;

use stdweb::web::{IParentNode, document};
use yew::prelude::*;
use yew::html::ComponentUpdate;
use yew::scheduler::Scheduler;
use two_apps::{Context, Model, Msg};

fn mount_app(selector: &'static str, app: App<Context, Model>) {
    let element = document().query_selector(selector).unwrap().unwrap();
    app.mount(element);
}

fn main() {
    yew::initialize();

    let context = Context::new();

    // Example how to reuse context in two scopes
    let scheduler = Scheduler::new(context);

    let first_app = App::reuse(&scheduler);
    let mut to_first = first_app.get_env();

    let second_app = App::reuse(&scheduler);
    let mut to_second = second_app.get_env();

    mount_app(".first-app", first_app);
    mount_app(".second-app", second_app);
    to_first.send(ComponentUpdate::Message(Msg::SetActivator(to_second.clone())));
    to_second.send(ComponentUpdate::Message(Msg::SetActivator(to_first.clone())));

    yew::run_loop();
}
