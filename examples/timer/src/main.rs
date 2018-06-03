extern crate yew;
extern crate timer;

use yew::prelude::*;
use yew::services::timeout::TimeoutService;
use yew::services::interval::IntervalService;
use yew::services::console::ConsoleService;
use timer::Model;

fn main() {
    yew::initialize();
    let app: App<_, Model> = App::new(());
    app.mount_to_body();
    yew::run_loop();
}
