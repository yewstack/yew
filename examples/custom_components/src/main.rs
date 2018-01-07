#[macro_use]
extern crate yew;

mod counter;

use std::rc::Rc;
use std::cell::RefCell;
use yew::html::*;
use yew::services::console::ConsoleService;
use yew::component::Component;
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

    fn update(&mut self, msg: Self::Msg, context: &mut Context) {
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

    fn view(&self) -> Html<Self::Msg, Context> {
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
    let mut app = App::new();
    let context = Context {
        console: ConsoleService,
    };
    let model = Model {
        value: 0,
    };
    let context = Rc::new(RefCell::new(context));
    app.mount(context, model);
    yew::run_loop();
}
