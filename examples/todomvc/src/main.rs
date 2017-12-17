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
    Add,
}

fn update(model: &mut Model, msg: Msg) {
    match msg {
        Add => {
            let entry = Entry {
                description: "Test".into(),
                completed: false,
            };
            model.entries.push(entry);
        }
    }
}

fn view(model: &Model) -> html::Html<Msg> {
    html! {
        <div class="todomvc-wrapper",>
            <section class="todoapp",>
                <header class="header",>
                    <h1>{ "Todos" }</h1>
                    { viewInput() }
                    { viewEntries(&model.entries) }
                </header>
            </section>
        </div>
    }
}

fn viewInput() -> html::Html<Msg> {
    html! {
        <input class="new-todo", (onclick)=|_| Msg::Add, />
    }
}

fn viewEntries(entries: &Vec<Entry>) -> html::Html<Msg> {
    html! {
        <section class="main",>
            { for entries.iter().map(viewEntry) }
        </section>
    }
}

fn viewEntry(entry: &Entry) -> html::Html<Msg> {
    html! {
        <li>{ &entry.description }</li>
    }
}

fn main() {
    let model = Model {
        entries: Vec::new(),
    };
    html::program(model, update, view);
}
