extern crate chrono;
#[macro_use]
extern crate yew;

use chrono::prelude::*;
use yew::html::*;
use yew::services::console::ConsoleService;

struct Context {
    console: ConsoleService,
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

    fn update(&mut self, msg: Msg, context: &mut ScopeRef<Context, Self>) {
        match msg {
            Msg::Increment => {
                self.value = self.value + 1;
                context.console.log("plus one");
            }
            Msg::Decrement => {
                self.value = self.value - 1;
                context.console.log("minus one");
            }
            Msg::Bulk(list) => {
                for msg in list {
                    self.update(msg, context);
                }
            }
        }
    }

    fn view(&self) -> Html<Context, Self> {
        html! {
            <div>
                <nav class="menu",>
                    <button onclick=|_| Msg::Increment,>{ "Increment" }</button>
                    <button onclick=|_| Msg::Decrement,>{ "Decrement" }</button>
                    <button onclick=|_| Msg::Bulk(vec!(Msg::Increment, Msg::Increment)),>{ "Increment Twice" }</button>
                </nav>
                <p>{ self.value }</p>
                <p>{ Local::now() }</p>
            </div>
        }
    }
}

fn main() {
    yew::initialize();
    let context = Context {
        console: ConsoleService,
    };
    let app: Scope<_, Model> = Scope::new(context);
    app.mount_to_body();
    yew::run_loop();
}
