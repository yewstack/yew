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
    with_barrier: bool,
    color: Color,
}

enum Msg {
    Repaint,
    Toggle,
    ChildClicked(u32),
}

impl Component<Context> for Model {
    type Msg = Msg;
    type Properties = ();

    fn create(_: &mut Env<Context, Self>) -> Self {
        Model {
            with_barrier: false,
            color: Color::Red,
        }
    }

    fn update(&mut self, msg: Msg, context: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::Repaint => {
                self.color = Color::Blue;
                true
            }
            Msg::Toggle => {
                self.with_barrier = !self.with_barrier;
                true
            }
            Msg::ChildClicked(value) => {
                context.console.log(&format!("child clicked: {}", value));
                false
            }
        }
    }
}

impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {
        let counter = |x| html! {
            <Counter: initial=x, color=&self.color, onclick=Msg::ChildClicked,/>
        };
        html! {
            <div class="custom-components-example",>
                <button onclick=|_| Msg::Toggle,>{ "Toggle" }</button>
                { self.view_barrier() }
                { for (1..1001).map(counter) }
            </div>
        }
    }
}

impl Model {
    fn view_barrier(&self) -> Html<Context, Self> {
        if self.with_barrier {
            html! {
                <Barrier: limit=10, onsignal=|_| Msg::Repaint, />
            }
        } else {
            html! {
                <p>{ "Click \"toggle\"!" }</p>
            }
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
