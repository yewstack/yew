#[macro_use]
extern crate yew;

use yew::html;

struct Model {
    entries: Vec<Entry>,
}

struct Entry {
    description: String,
    completed: bool,
}

enum Msg {
}

fn update(model: &mut Model, msg: Msg) {
}

fn view(model: &Model) -> html::Html<Msg> {
    html! {
        <div class="todomvc-wrapper",>
            <section class="todoapp",>
                <header class="header",>
                    <h1>{ "Todos" }</h1>
                    <input class="new-todo",/>
                </header>
            </section>
        </div>
    }
}

fn main() {
    let model = Model {
        entries: Vec::new(),
    };
    html::program(model, update, view);
}
