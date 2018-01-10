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
    value: i64,
}

enum Msg {
    Increment,
    Decrement,
    Bulk(Vec<Msg>),
}


impl Component<Context> for Model {
    type Msg = Msg;
    type Properties = ();

    fn create(_: &mut ScopeRef<Context, Self>) -> Self {
        Model { value: 0 }
    }

    fn update(&mut self, msg: Self::Msg, context: &mut ScopeRef<Context, Self>) {
        match msg {
            Msg::Increment => {
                self.value = self.value + 1;
            }
            Msg::Decrement => {
                self.value = self.value - 1;
            }
            Msg::Bulk(list) => {
                for msg in list {
                    self.update(msg, context);
                }
            }
        }
    }

    fn view(&self) -> Html<Context, Self> {
        let counter = |_| html! {
            <Counter: color=Color::Red,/>
        };
        html! {
            <div>
                <nav class="menu",>
                    { for (1..1000).map(counter) }
                    <button onclick=|_| Msg::Increment,>{ "Increment" }</button>
                    <button onclick=|_| Msg::Decrement,>{ "Decrement" }</button>
                    <button onclick=|_| Msg::Bulk(vec!(Msg::Increment, Msg::Increment)),>{ "Increment Twice" }</button>
                </nav>
                <p>{ self.value }</p>
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
