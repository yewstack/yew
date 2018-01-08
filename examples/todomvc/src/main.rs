#![recursion_limit="128"]

extern crate strum;
#[macro_use]
extern crate strum_macros;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate yew;

use strum::IntoEnumIterator;
use yew::html::*;
use yew::format::Json;
use yew::services::storage::{StorageService, Area};

const KEY: &'static str = "yew.todomvc.model";

struct Context {
    storage: StorageService,
}

#[derive(Serialize, Deserialize)]
struct Model {
    entries: Vec<Entry>,
    filter: Filter,
    value: String,
    edit_value: String,
}

#[derive(Serialize, Deserialize)]
struct Entry {
    description: String,
    completed: bool,
    editing: bool,
}

enum Msg {
    Add,
    Edit(usize),
    Update(String),
    UpdateEdit(String),
    Remove(usize),
    SetFilter(Filter),
    ToggleAll,
    ToggleEdit(usize),
    Toggle(usize),
    ClearCompleted,
    Nope,
}

impl Component<Context> for Model {
    type Msg = Msg;

    fn create(context: &mut ScopeRef<Context, Msg>) -> Self {
        if let Json(Ok(restored_model)) = context.storage.restore(KEY) {
            restored_model
        } else {
            Model {
                entries: Vec::new(),
                filter: Filter::All,
                value: "".into(),
                edit_value: "".into(),
            }
        }
    }

    fn update(&mut self, msg: Msg, context: &mut ScopeRef<Context, Msg>) {
        match msg {
            Msg::Add => {
                let entry = Entry {
                    description: self.value.clone(),
                    completed: false,
                    editing: false,
                };
                self.entries.push(entry);
                self.value = "".to_string();
            }
            Msg::Edit(idx) => {
                let edit_value = self.edit_value.clone();
                self.complete_edit(idx, edit_value);
                self.edit_value = "".to_string();
            }
            Msg::Update(val) => {
                println!("Input: {}", val);
                self.value = val;
            }
            Msg::UpdateEdit(val) => {
                println!("Input: {}", val);
                self.edit_value = val;
            }
            Msg::Remove(idx) => {
                self.remove(idx);
            }
            Msg::SetFilter(filter) => {
                self.filter = filter;
            }
            Msg::ToggleEdit(idx) => {
                self.toggle_edit(idx);
            }
            Msg::ToggleAll => {
                let status = !self.is_all_completed();
                self.toggle_all(status);
            }
            Msg::Toggle(idx) => {
                self.toggle(idx);
            }
            Msg::ClearCompleted => {
                self.clear_completed();
            }
            Msg::Nope => {}
        }
        context.storage.store(KEY, Json(&self));
    }

    fn view(&self) -> Html<Context, Msg> {
        html! {
            <div class="todomvc-wrapper",>
                <section class="todoapp",>
                    <header class="header",>
                        <h1>{ "todos" }</h1>
                        { self.view_input() }
                    </header>
                    <section class="main",>
                        <input class="toggle-all", type="checkbox", checked=self.is_all_completed(), onclick=|_| Msg::ToggleAll, />
                        <ul class="todo-list",>
                            { for self.entries.iter().filter(|e| self.filter.fit(e)).enumerate().map(view_entry) }
                        </ul>
                    </section>
                    <footer class="footer",>
                        <span class="todo-count",>
                            <strong>{ self.total() }</strong>
                            { " item(s) left" }
                        </span>
                        <ul class="filters",>
                            { for Filter::iter().map(|flt| self.view_filter(flt)) }
                        </ul>
                        <button class="clear-completed", onclick=|_| Msg::ClearCompleted,>
                            { format!("Clear completed ({})", self.total_completed()) }
                        </button>
                    </footer>
                </section>
                <footer class="info",>
                    <p>{ "Double-click to edit a todo" }</p>
                    <p>{ "Written by " }<a href="https://github.com/DenisKolodin/", target="_blank",>{ "Denis Kolodin" }</a></p>
                    <p>{ "Part of " }<a href="http://todomvc.com/", target="_blank",>{ "TodoMVC" }</a></p>
                </footer>
            </div>
        }
    }
}

impl Model {
    fn view_filter(&self, filter: Filter) -> Html<Context, Msg> {
        let flt = filter.clone();
        html! {
            <li>
                <a class=if self.filter == flt { "selected" } else { "not-selected" },
                   href=&flt,
                   onclick=move |_| Msg::SetFilter(flt.clone()),>
                    { filter }
                </a>
            </li>
        }
    }

    fn view_input(&self) -> Html<Context, Msg> {
        html! {
            // You can use standard Rust comments. One line:
            // <li></li>
            <input class="new-todo",
                   placeholder="What needs to be done?",
                   value=&self.value,
                   oninput=|e: InputData| Msg::Update(e.value),
                   onkeypress=|e: KeyData| {
                       if e.key == "Enter" { Msg::Add } else { Msg::Nope }
                   }, />
            /* Or multiline:
            <ul>
                <li></li>
            </ul>
            */
        }
    }
}

fn view_entry((idx, entry): (usize, &Entry)) -> Html<Context, Msg> {
    html! {
        <li class=if entry.editing == true { "editing" } else { "" },>
            <div class="view",>
                <input class="toggle", type="checkbox", checked=entry.completed, onclick=move|_| Msg::Toggle(idx), />
                <label ondoubleclick=move|_| Msg::ToggleEdit(idx),>{ &entry.description }</label>
                <button class="destroy", onclick=move |_| Msg::Remove(idx), />
            </div>
            { view_entry_edit_input((idx, &entry)) }
        </li>
    }
}

fn view_entry_edit_input((idx, entry): (usize, &Entry)) -> Html<Context, Msg> {
    if entry.editing == true {
        html! {
            <input class="edit",
                   type="text",
                   value=&entry.description,
                   oninput=|e: InputData| Msg::UpdateEdit(e.value),
                   onkeypress=move |e: KeyData| {
                      if e.key == "Enter" { Msg::Edit(idx) } else { Msg::Nope }
                   }, />
        }
    } else {
        html! { <input type="hidden", /> }
    }
}

fn main() {
    yew::initialize();
    let context = Context {
        storage: StorageService::new(Area::Local),
    };
    let app = Scope::new(context);
    app.mount_to_body::<Model>();
    yew::run_loop();
}

#[derive(EnumIter, ToString, Clone, PartialEq)]
#[derive(Serialize, Deserialize)]
enum Filter {
    All,
    Active,
    Completed,
}

impl<'a> Into<Href> for &'a Filter {
    fn into(self) -> Href {
        match *self {
            Filter::All => "#/".into(),
            Filter::Active => "#/active".into(),
            Filter::Completed => "#/completed".into(),
        }
    }
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

impl Model {
    fn total(&self) -> usize {
        self.entries.len()
    }

    fn total_completed(&self) -> usize {
        self.entries.iter().filter(|e| Filter::Completed.fit(e)).count()
    }

    fn is_all_completed(&self) -> bool {
        let entries = self.entries.iter()
            .filter(|e| self.filter.fit(e))
            .collect::<Vec<_>>();
        if entries.len() == 0 {
            false
        } else {
            entries.into_iter()
                .fold(true, |status, entry| status && entry.completed)
        }
    }

    fn toggle_all(&mut self, value: bool) {
        for entry in self.entries.iter_mut() {
            if self.filter.fit(entry) {
                entry.completed = value;
            }
        }
    }

    fn clear_completed(&mut self) {
        let entries = self.entries.drain(..)
            .filter(|e| Filter::Active.fit(e))
            .collect();
        self.entries = entries;
    }

    fn toggle(&mut self, idx: usize) {
        let filter = self.filter.clone();
        let mut entries = self.entries
            .iter_mut()
            .filter(|e| filter.fit(e))
            .collect::<Vec<_>>();
        let entry = entries.get_mut(idx).unwrap();
        entry.completed = !entry.completed;
    }

    fn toggle_edit(&mut self, idx: usize) {
        let filter = self.filter.clone();
        let mut entries = self.entries
            .iter_mut()
            .filter(|e| filter.fit(e))
            .collect::<Vec<_>>();
        let entry = entries.get_mut(idx).unwrap();
        entry.editing = !entry.editing;
    }

    fn complete_edit(&mut self, idx: usize, val: String) {
        let filter = self.filter.clone();
        let mut entries = self.entries
            .iter_mut()
            .filter(|e| filter.fit(e))
            .collect::<Vec<_>>();
        let entry = entries.get_mut(idx).unwrap();
        entry.description = val;
        entry.editing = !entry.editing;
    }

    fn remove(&mut self, idx: usize) {
        let idx = {
            let filter = self.filter.clone();
            let entries = self.entries
                .iter()
                .enumerate()
                .filter(|&(_, e)| filter.fit(e))
                .collect::<Vec<_>>();
            let &(idx, _) = entries.get(idx).unwrap();
            idx
        };
        self.entries.remove(idx);
    }
}
