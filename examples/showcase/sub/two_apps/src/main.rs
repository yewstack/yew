extern crate stdweb;
extern crate yew;
extern crate two_apps;

use std::rc::Rc;
use std::cell::RefCell;
use stdweb::web::{IParentNode, document};
// Use `html` module directly. No use `App`.
use yew::html::*;
use two_apps::{Context, Model};

fn mount_app(selector: &'static str, app: Scope<Context, Model>) {
    let element = document().query_selector(selector).unwrap().unwrap();
    app.mount(element);
}

fn main() {
    yew::initialize();

    let context = Context::new();

    // Example how to reuse context in two scopes
    let context = Rc::new(RefCell::new(context));

    let mut first_app = Scope::reuse(context.clone());
    let to_first = first_app.get_env().sender();
    context.borrow_mut().senders.push(to_first);

    let mut second_app = Scope::reuse(context.clone());
    let to_second = second_app.get_env().sender();
    context.borrow_mut().senders.push(to_second);

    mount_app(".first-app", first_app);
    mount_app(".second-app", second_app);

    yew::run_loop();
}
