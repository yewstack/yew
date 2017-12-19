#![recursion_limit="128"]

#[macro_use]
extern crate yew;

use yew::html::*;

enum Filter {
    All,
    Active,
    Completed,
}

impl ToString for Filter {
    fn to_string(&self) -> String {
        let name = match *self {
            Filter::All => "All",
            Filter::Active => "Active",
            Filter::Completed => "Completed",
        };
        name.to_string()
    }
}

struct Model {
    entries: Vec<Entry>,
    filter: Filter,
    value: String,
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
    Update(String),
    Remove(usize),
    SetFilter(Filter),
    Nope,
}

fn update(model: &mut Model, msg: Msg) {
    match msg {
        Msg::Add => {
            let entry = Entry {
                description: model.value.clone(),
                completed: false,
            };
            model.entries.push(entry);
        }
        Msg::Update(val) => {
            println!("Input: {}", val);
            model.value = val;
        }
        Msg::Remove(idx) => {
            model.entries.remove(idx);
        }
        Msg::SetFilter(filter) => {
            model.filter = filter;
        }
        Msg::Nope => {
        }
    }
}

fn view(model: &Model) -> Html<Msg> {
    html! {
        <div class="todomvc-wrapper",>
            <section class="todoapp",>
                <header class="header",>
                    <h1>{ "todos" }</h1>
                    { view_input(&model) }
                </header>
                <section class="main",>
                    <input class="toggle-all", type="checkbox",/>
                    { view_entries(&model.entries) }
                </section>
                <footer class="footer",>
                    <span class="todo-count",>
                        <strong>{ model.total() }</strong>
                        { " item(s) left" }
                    </span>
                    <ul class="filters",>
                        <li>
                            <a onclick=|_| Msg::SetFilter(Filter::All),>
                                { Filter::All }
                            </a>
                        </li>
                        <li>
                            <a onclick=|_| Msg::SetFilter(Filter::Active),>
                                { Filter::Active }
                            </a>
                        </li>
                        <li>
                            <a onclick=|_| Msg::SetFilter(Filter::Completed),>
                                { Filter::Completed }
                            </a>
                        </li>
                    </ul>
                    <button class="clear-completed",>
                        { format!("Clear completed ({})", model.total_completed()) }
                    </button>
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

fn view_input(model: &Model) -> Html<Msg> {
    html! {
        <input class="new-todo",
               placeholder="What needs to be done?",
               value=&model.value,
               oninput=|e: InputData| Msg::Update(e.value),
               onkeypress=|e: KeyData| {
                   if e.key == "Enter" { Msg::Add } else { Msg::Nope }
               }, />
    }
}

fn view_entries(entries: &Vec<Entry>) -> Html<Msg> {
    html! {
        <ul class="todo-list",>
            { for entries.iter().enumerate().map(view_entry) }
            // You can use standard Rust comments. One line:
            // <li></li>
        </ul>
        /* Or multiline:
        <ul>
            <li></li>
        </ul>
        */
    }
}

fn view_entry((idx, entry): (usize, &Entry)) -> Html<Msg> {
    html! {
        <li>
            <div class="view",>
                <input class="toggle", type="checkbox", />
                <label>{ &entry.description }</label>
                <button class="destroy", onclick=move |_| Msg::Remove(idx),></button>
            </div>
        </li>
    }
}

fn main() {
    let model = Model {
        entries: Vec::new(),
        filter: Filter::All,
        value: "".into(),
    };
    program(model, update, view);
}
