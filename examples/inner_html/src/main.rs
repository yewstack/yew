#![recursion_limit="512"]
extern crate stdweb;
extern crate yew;
extern crate inner_html;

use yew::prelude::*;
use yew::services::console::ConsoleService;

use inner_html::Model;

pub struct Context {
    console: ConsoleService,
}

impl AsMut<ConsoleService> for Context {
    fn as_mut(&mut self) -> &mut ConsoleService {
        &mut self.console
    }
}

fn main() {
    yew::initialize();
    let context = Context {
        console: ConsoleService::new(),
    };
    let app: App<_, Model> = App::new(context);
    app.mount_to_body();
    yew::run_loop();
}
