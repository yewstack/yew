#![recursion_limit="128"]

extern crate strum;
#[macro_use]
extern crate strum_macros;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate yew;

use strum::IntoEnumIterator;
use yew::prelude::*;
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

fn update(context: &mut AppContext<Context, Model, Msg>, model: &mut Model, msg: Msg) -> ShouldRender {
    match msg {
        Msg::Add => {
            let entry = Entry {
                description: model.value.clone(),
                completed: false,
                editing: false,
            };
            model.entries.push(entry);
            model.value = "".to_string();
        }
        Msg::Edit(idx) => {
            let edit_value = model.edit_value.clone();
            model.complete_edit(idx, edit_value);
            model.edit_value = "".to_string();
        }
        Msg::Update(val) => {
            println!("Input: {}", val);
            model.value = val;
        }
        Msg::UpdateEdit(val) => {
            println!("Input: {}", val);
            model.edit_value = val;
        }
        Msg::Remove(idx) => {
            model.remove(idx);
        }
        Msg::SetFilter(filter) => {
            model.filter = filter;
        }
        Msg::ToggleEdit(idx) => {
            model.edit_value = model.entries[idx].description.clone();
            model.toggle_edit(idx);
        }
        Msg::ToggleAll => {
            let status = !model.is_all_completed();
            model.toggle_all(status);
        }
        Msg::Toggle(idx) => {
            model.toggle(idx);
        }
        Msg::ClearCompleted => {
            model.clear_completed();
        }
        Msg::Nope => {}
    }
    context.storage.store(KEY, Json(&model));
    true
}

fn view(model: &Model) -> AppHtml<Context, Model, Msg> {
    html! {
        <div class="todomvc-wrapper",>
            <section class="todoapp",>
                <header class="header",>
                    <h1>{ "todos" }</h1>
                    { view_input(&model) }
                </header>
                <section class="main",>
                    <input class="toggle-all", type="checkbox", checked=model.is_all_completed(), onclick=|_| Msg::ToggleAll, />
                    <ul class="todo-list",>
                        { for model.entries.iter().filter(|e| model.filter.fit(e)).enumerate().map(view_entry) }
                    </ul>
                </section>
                <footer class="footer",>
                    <span class="todo-count",>
                        <strong>{ model.total() }</strong>
                        { " item(s) left" }
                    </span>
                    <ul class="filters",>
                        { for Filter::iter().map(|flt| view_filter(&model, flt)) }
                    </ul>
                    <button class="clear-completed", onclick=|_| Msg::ClearCompleted,>
                        { format!("Clear completed ({})", model.total_completed()) }
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

fn view_filter(model: &Model, filter: Filter) -> AppHtml<Context, Model, Msg> {
    let flt = filter.clone();
    html! {
        <li>
            <a class=if model.filter == flt { "selected" } else { "not-selected" },
               href=&flt,
               onclick=move |_| Msg::SetFilter(flt.clone()),>
                { filter }
            </a>
        </li>
    }
}

fn view_input(model: &Model) -> AppHtml<Context, Model, Msg> {
    html! {
        // You can use standard Rust comments. One line:
        // <li></li>
        <input class="new-todo",
               placeholder="What needs to be done?",
               value=&model.value,
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

fn view_entry((idx, entry): (usize, &Entry)) -> AppHtml<Context, Model, Msg> {
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

fn view_entry_edit_input((idx, entry): (usize, &Entry)) -> AppHtml<Context, Model, Msg> {
    if entry.editing == true {
        html! {
            <input class="edit",
                   type="text",
                   value=&entry.description,
                   oninput=|e: InputData| Msg::UpdateEdit(e.value),
                   onblur=move|_| Msg::Edit(idx),
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
    let app = App::new();
    let mut context = Context {
        storage: StorageService::new(Area::Local),
    };
    let model = {
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
    };
    app.mount(context, model, update, view);
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
