#![recursion_limit="128"]

#[macro_use]
extern crate yew;

use yew::html::*;

#[derive(Clone)]
enum Filter {
    All,
    Active,
    Completed,
}

impl Filter {
    fn fit(&self, entry: &Entry) -> bool {
        match *self {
            Filter::All => true,
            Filter::Active => !entry.completed,
            Filter::Completed => entry.completed,
        }
    }
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
        self.entries.iter().filter(|e| Filter::Completed.fit(e)).count()
    }

    fn is_all_completed(&self) -> bool {
        self.entries.iter()
            .filter(|e| self.filter.fit(e))
            .fold(true, |status, entry| status && entry.completed)
    }

    fn complete_all(&mut self) {
        for entry in self.entries.iter_mut() {
            if self.filter.fit(entry) {
                entry.completed = true;
            }
        }
    }

    fn clear_completed(&mut self) {
        let entries = self.entries.drain(..)
            .filter(|e| Filter::Active.fit(e))
            .collect();
        self.entries = entries;
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
    SelectAll,
    Toggle(usize),
    ClearCompleted,
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
            model.value = "".to_string();
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
        Msg::SelectAll => {
            model.complete_all();
        }
        Msg::Toggle(idx) => {
            let entry = model.entries.get_mut(idx).unwrap();
            entry.completed = !entry.completed;
        }
        Msg::ClearCompleted => {
            model.clear_completed();
        }
        Msg::Nope => {}
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
                    <input class="toggle-all", type="checkbox", onclick=|_| Msg::SelectAll, />
                    { view_entries(&model) }
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
                    <button class="clear-completed", onclick=|_| Msg::ClearCompleted,>
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

fn view_entries(model: &Model) -> Html<Msg> {
    html! {
        <ul class="todo-list",>
            { for model.entries.iter().filter(|e| model.filter.fit(e)).enumerate().map(view_entry) }
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
                <input class="toggle", type="checkbox", checked=entry.completed, oninput=move|_| Msg::Toggle(idx), />
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
