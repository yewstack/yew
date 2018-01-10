#[macro_use]
extern crate yew;

mod counter;

use yew::html::*;
use yew::services::console::ConsoleService;
use counter::{Counter, Color};

struct Context {
    console: ConsoleService,
}

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
}


impl Component<Context> for Model {
    type Msg = Msg;
    type Properties = ();

    fn create(_: &mut ScopeRef<Context, Self>) -> Self {
        Model { color: Color::Red }
    }

    fn update(&mut self, msg: Self::Msg, _: &mut ScopeRef<Context, Self>) -> ShouldUpdate {
        match msg {
            Msg::Repaint => {
                self.color = Color::Blue;
            }
        }
        true
    }

    fn view(&self) -> Html<Context, Self> {
        let counter = |_| html! {
            <Counter: color=&self.color, onclick=|v| println!("FIRED! {}", v),/>
        };
        html! {
            <div>
                <button onclick=|_| Msg::Repaint,>{ "Repaint" }</button>
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
    let app: Scope<Context, Model> = Scope::new(context);
    app.mount_to_body();
    yew::run_loop();
}
