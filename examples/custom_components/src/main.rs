#[macro_use]
extern crate yew;

mod counter;
mod button;
mod barrier;

use yew::prelude::*;
use yew::html::Scope;
use yew::services::console::ConsoleService;
use counter::{Counter, Color};
use barrier::Barrier;

struct Context {
    console: ConsoleService,
}

/// If you use `App` you should implement this for `AppContext<Context, Model, Msg>` struct.
impl counter::Printer for Context {
    fn print(&mut self, data: &str) {
        self.console.log(data);
    }
}

struct Model {
    color: Color,
}

enum Msg {
    Repaint,
    ChildClicked(u32),
}

impl Component<Context> for Model {
    type Msg = Msg;
    type Properties = ();

    fn create(_: &mut ScopeRef<Context, Self>) -> Self {
        Model {
            color: Color::Red,
        }
    }

    fn update(&mut self, msg: Msg, context: &mut ScopeRef<Context, Self>) -> ShouldUpdate {
        match msg {
            Msg::Repaint => {
                self.color = Color::Blue;
                true
            }
            Msg::ChildClicked(value) => {
                context.console.log(&format!("child clicked: {}", value));
                false
            }
        }
    }

    fn view(&self) -> Html<Context, Self> {
        let counter = |_| html! {
            <Counter: color=&self.color, onclick=Msg::ChildClicked,/>
        };
        html! {
            <div>
                <Barrier: limit=10, onsignal=|_| Msg::Repaint, />
                { for (0..1000).map(counter) }
            </div>
        }
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
