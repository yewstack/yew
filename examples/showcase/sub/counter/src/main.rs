extern crate yew;
extern crate counter;

use yew::prelude::*;
use yew::services::console::ConsoleService;
use counter::Model;

pub struct Context {
    console: ConsoleService,
}

impl AsRef<ConsoleService> for Context {
    fn as_ref(&self) -> &ConsoleService {
        &self.console
    }
}

fn main() {
    yew::initialize();
    let context = Context {
        console: ConsoleService,
    };
    let app: App<_, Model> = App::new(context);
    app.mount_to_body();
    yew::run_loop();
}
