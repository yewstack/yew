extern crate yew;
extern crate custom_components;

use yew::html::Scope;
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
        console: ConsoleService,
    };
    // We use `Scope` here for demonstration.
    // You can also use `App` here too.
    let app: Scope<Context, Model> = Scope::new(context);
    app.mount_to_body();
    yew::run_loop();
}
