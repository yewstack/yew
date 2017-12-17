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
                    <= viewInput() =>
                    <= viewEntries(&model.entries) =>
                </header>
            </section>
        </div>
    }
}

fn viewInput() -> html::Html<Msg> {
    html! {
        <input class="new-todo",/>
    }
}

fn viewEntries(entries: &Vec<Entry>) -> html::Html<Msg> {
    html! {
        <section class="main",>
        </section>
    }
}

fn main() {
    let model = Model {
        entries: Vec::new(),
    };
    html::program(model, update, view);
}
