#[macro_use]
extern crate yew;

mod counter;

use yew::html::*;
use yew::services::console::ConsoleService;
use yew::component::Component;
use counter::Counter;

struct Context {
    console: ConsoleService,
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


impl Component for Model {
    type Msg = Msg;

    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Increment => {
                self.value = self.value + 1;
            }
            Msg::Decrement => {
                self.value = self.value - 1;
            }
            Msg::Bulk(list) => {
                for msg in list {
                    self.update(msg);
                }
            }
        }
    }

    fn view(&self) -> Html<Msg> {
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
    app.mount(context, model);
    yew::run_loop();
}
