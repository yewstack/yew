extern crate yew;
extern crate custom_components;

use yew::prelude::*;
use yew::services::console::ConsoleService;
use custom_components::{Printer, Model};

struct Context {
    console: ConsoleService,
}

/// If you use `App` you should implement this for `AppContext<Context, Model, Msg>` struct.
impl Printer for Context {
    fn print(&mut self, data: &str) {
        self.console.log(data);
    }
}

fn main() {
    yew::initialize();
    let context = Context {
        console: ConsoleService::new(),
    };
    let app: App<Context, Model> = App::new(context);
    app.mount_to_body();
    yew::run_loop();
}
