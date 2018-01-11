#[macro_use]
extern crate yew;

mod counter;
mod button;
mod barrier;

use yew::prelude::*;
use yew::services::console::ConsoleService;
use counter::{Counter, Color};
use barrier::Barrier;

struct Context {
    console: ConsoleService,
}

impl counter::Printer for AppContext<Context, Model, Msg> {
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

fn update(context: &mut Context, model: &mut Model, msg: Msg) -> ShouldUpdate {
    match msg {
        Msg::Repaint => {
            model.color = Color::Blue;
            true
        }
        Msg::ChildClicked(value) => {
            context.console.log(&format!("child clicked: {}", value));
            false
        }
    }
}

fn view(model: &Model) -> AppHtml<Context, Model, Msg> {
    let counter = |_| html! {
        <Counter: color=&model.color, onclick=Msg::ChildClicked,/>
    };
    html! {
        <div>
            <Barrier: limit=10, onsignal=|_| Msg::Repaint, />
            { for (0..1000).map(counter) }
        </div>
    }
}

fn main() {
    yew::initialize();
    let app = App::new();
    let context = Context {
        console: ConsoleService,
    };
    let model = Model {
        color: Color::Red,
    };
    app.mount(context, model, update, view);
    yew::run_loop();
}
