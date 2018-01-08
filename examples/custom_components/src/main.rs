#[macro_use]
extern crate yew;

mod counter;

use yew::html::*;
use yew::services::console::ConsoleService;
use counter::Counter;

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

impl Default for Model {
    fn default() -> Self {
        Model {
            value: 0,
        }
    }
}

enum Msg {
    Increment,
    Decrement,
    Bulk(Vec<Msg>),
}


impl Component<Context> for Model {
    type Msg = Msg;

    fn update(&mut self, msg: Self::Msg, context: &mut ScopeRef<Context, Msg>) {
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

    fn view(&self) -> Html<Context, Self::Msg> {
        let counter = |_| html! {
            <Counter: />
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
    let mut app = Scope::new(context);
    app.mount(Model::default());
    yew::run_loop();
}
