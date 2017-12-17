#![recursion_limit="128"]

#[macro_use]
extern crate yew;

use yew::html;

struct Model {
    entries: Vec<Entry>,
}

impl Model {
    fn total(&self) -> usize {
        self.entries.len()
    }

    fn total_completed(&self) -> usize {
        self.entries.iter().filter(|entry| entry.completed).count()
    }
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
                    <h1>{ "todos" }</h1>
                    { view_input() }
                    { view_entries(&model.entries) }
                </header>
                <section class="main",>
                </section>
                <footer class="footer",>
                    <span class="todo-count",>
                        <strong>{ model.total() }</strong>
                        { " item(s) left" }
                    </span>
                    <ul class="filters",>
                        <li><a>{ "All" }</a></li>
                        <li><a>{ "Active" }</a></li>
                        <li><a>{ "Completed" }</a></li>
                    </ul>
                    <button class="clear-completed",>{ format!("Clear completed ({})", model.total_completed()) }</button>
                </footer>
            </section>
            <footer class="info",>
                <p>{ "Double-click to edit a todo" }</p>
                <p>{ "Written by " }<a>{ "Denis Kolodin" }</a></p>
                <p>{ "Part of " }<a>{ "TodoMVC" }</a></p>
            </footer>
        </div>
    }
}

fn view_input() -> html::Html<Msg> {
    html! {
        <input class="new-todo", (onclick)=|_| Msg::Add, />
    }
}

fn view_entries(entries: &Vec<Entry>) -> html::Html<Msg> {
    html! {
        <section class="main",>
            { for entries.iter().map(view_entry) }
        </section>
    }
}

fn view_entry(entry: &Entry) -> html::Html<Msg> {
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
