#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate yew;

use yew::prelude::*;
use yew::services::console::ConsoleService;
use yew::services::router::{RouteInfo, RouterTask};

use stdweb::web::Date;

struct Context {
    console: ConsoleService,
}

struct Model {
    value: i64,
    router: RouterTask<Context, Model>,
}

enum Msg {
    Increment,
    Decrement,
    None,
    Bulk(Vec<Msg>),
}

impl Component<Context> for Model {
    type Msg = Msg;
    type Properties = ();

    fn create(_: Self::Properties, context: &mut Env<Context, Self>) -> Self {
        Model {
            value: 0,
            router: RouterTask::new(context, &handle_route),
        }
    }

    fn update(&mut self, msg: Self::Msg, context: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::Increment => {
                self.value = self.value + 1;
                context.console.log("plus one");
            }
            Msg::Decrement => {
                self.value = self.value - 1;
                context.console.log("minus one");
            }
            Msg::Bulk(list) => for msg in list {
                self.update(msg, context);
                context.console.log("Bulk action");
            },
            Msg::None => {
                context.console.log("No action");
                return false;
            }
        }
        true
    }
}

impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {
        html! {
            <div>
                <p>
                    <span>{ "Use the buttons below or go to "}</span>
                    <a href="#increment",>{"#increment"}</a>
                    <span>{" or "}</span>
                    <a href="#decrement",>{"#decrement"}</a>
                </p>
                <nav class="menu",>
                    <button onclick=|_| Msg::Increment,>{ "Increment" }</button>
                    <button onclick=|_| Msg::Decrement,>{ "Decrement" }</button>
                    <button onclick=|_| Msg::Bulk(vec![Msg::Increment, Msg::Increment]),>{ "Increment Twice" }</button>
                </nav>
                <p>{ self.value }</p>
                <p>{ Date::new().to_string() }</p>
            </div>
        }
    }
}

fn handle_route(info: RouteInfo) -> Msg {
    let route = info.url.fragment().unwrap_or("");
    println!("Handling route: {}", route);
    if route.to_ascii_lowercase() == "increment" {
        Msg::Increment
    } else if route.to_ascii_lowercase() == "decrement" {
        Msg::Decrement
    } else {
        Msg::None
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
