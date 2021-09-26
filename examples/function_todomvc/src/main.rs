use gloo::storage::{LocalStorage, Storage};
use state::{Entry, Filter, State};
use std::rc::Rc;
use strum::IntoEnumIterator;
use web_sys::HtmlInputElement as InputElement;
use yew::events::KeyboardEvent;
use yew::{
    classes, function_component, html, use_effect_with_deps, use_reducer, Callback, TargetCast,
};

mod components;
mod state;

use components::entry::Entry as EntryItem;
use components::filter::Filter as FilterItem;
use components::info_footer::InfoFooter;

pub enum Action {
    Add(String),
    Edit((usize, String)),
    Remove(usize),
    SetFilter(Filter),
    ToggleAll,
    ToggleEdit(usize),
    Toggle(usize),
    ClearCompleted,
    // Focus,
}

const KEY: &str = "yew.functiontodomvc.self";

#[function_component(App)]
fn app() -> Html {
    let state = use_reducer(
        |prev: Rc<State>, action: Action| match action {
            Action::Add(description) => {
                let mut entries = prev.entries.clone();
                entries.push(Entry {
                    id: entries.last().map(|entry| entry.id + 1).unwrap_or(1),
                    description,
                    completed: false,
                    editing: false,
                });
                State {
                    entries,
                    filter: prev.filter,
                }
            }
            Action::Remove(id) => {
                let mut entries = prev.entries.clone();
                entries.retain(|entry| entry.id != id);
                State {
                    entries,
                    filter: prev.filter,
                }
            }
            Action::Toggle(id) => {
                let mut entries = prev.entries.clone();
                let entry = entries.iter_mut().find(|entry| entry.id == id);
                if let Some(entry) = entry {
                    entry.completed = !entry.completed;
                }
                State {
                    entries,
                    filter: prev.filter,
                }
            }
            Action::Edit((id, description)) => {
                let mut entries = prev.entries.clone();

                if description.is_empty() {
                    entries.retain(|entry| entry.id != id)
                }

                let entry = entries.iter_mut().find(|entry| entry.id == id);
                if let Some(entry) = entry {
                    entry.description = description;
                    entry.editing = false;
                }
                State {
                    entries,
                    filter: prev.filter,
                }
            }
            Action::ToggleEdit(id) => {
                let mut entries = prev.entries.clone();
                let entry = entries.iter_mut().find(|entry| entry.id == id);
                if let Some(entry) = entry {
                    entry.editing = !entry.editing;
                }
                State {
                    entries,
                    filter: prev.filter,
                }
            }
            Action::ToggleAll => {
                let mut entries = prev.entries.clone();
                for entry in &mut entries {
                    if prev.filter.fits(entry) {
                        entry.completed = !entry.completed;
                    }
                }
                State {
                    entries,
                    filter: prev.filter,
                }
            }
            Action::ClearCompleted => {
                let mut entries = prev.entries.clone();
                entries.retain(|e| Filter::Active.fits(e));
                State {
                    entries,
                    filter: prev.filter,
                }
            }
            Action::SetFilter(filter) => State {
                filter,
                entries: prev.entries.clone(),
            },
        },
        // Initial state
        State {
            entries: LocalStorage::get(KEY).unwrap_or_else(|_| vec![]),
            filter: Filter::All, // TODO: get from uri
        },
    );

    // Effect
    use_effect_with_deps(
        {
            let state = state.clone();
            move |_| {
                LocalStorage::set(KEY, &state.clone().entries).expect("failed to set");
                || ()
            }
        },
        state.clone(),
    );

    // Callbacks
    let onkeypress = {
        let state = state.clone();
        move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                let input: InputElement = e.target_unchecked_into();
                let value = input.value();
                input.set_value("");
                state.dispatch(Action::Add(value));
            }
        }
    };

    let remove_onclick = {
        let state = state.clone();
        Callback::from(move |id: usize| state.dispatch(Action::Remove(id)))
    };

    let toggle_onclick = {
        let state = state.clone();
        Callback::from(move |id: usize| state.dispatch(Action::Toggle(id)))
    };

    let toggle_all_onclick = {
        let state = state.clone();
        Callback::from(move |_| state.dispatch(Action::ToggleAll))
    };

    let toggle_edit_onclick = {
        let state = state.clone();
        Callback::from(move |id: usize| state.dispatch(Action::ToggleEdit(id)))
    };

    let clear_completed_onclick = {
        let state = state.clone();
        Callback::from(move |_| state.dispatch(Action::ClearCompleted))
    };

    let edit = {
        let state = state.clone();
        Callback::from(move |(id, value): (usize, String)| {
            state.dispatch(Action::Edit((id, value)));
        })
    };

    let set_filter_onclick = {
        let state = state.clone();
        Callback::from(move |filter: Filter| {
            state.dispatch(Action::SetFilter(filter));
        })
    };

    // Helpers
    let completed = {
        let state = state.clone();
        state
            .entries
            .iter()
            .filter(|entry| Filter::Completed.fits(entry))
            .count()
    };

    let is_all_completed = {
        let state = state.clone();
        let mut filtered_iter = state
            .entries
            .iter()
            .filter(|e| state.filter.fits(e))
            .peekable();

        if filtered_iter.peek().is_none() {
            false
        } else {
            filtered_iter.all(|entry| entry.completed)
        }
    };

    let total = {
        let state = state.clone();
        state.entries.len()
    };

    let hidden_class = if state.entries.is_empty() {
        "hidden"
    } else {
        ""
    };

    html! {
        <div class="todomvc-wrapper">
            <section class="todoapp">
                <header class="header">
                    <h1>{ "todos" }</h1>
                    <input
                        class="new-todo"
                        placeholder="What needs to be done?"
                        {onkeypress}
                    />
                </header>
                <section class={classes!("main", hidden_class)}>
                    <input
                        type="checkbox"
                        class="toggle-all"
                        id="toggle-all"
                        checked={is_all_completed}
                        onclick={toggle_all_onclick}
                    />
                    <label for="toggle-all" />
                    <ul class="todo-list">
                        { for state.entries.iter().filter(|e| state.filter.fits(e)).map(|entry|
                            html! {
                                <EntryItem entry={entry.clone()} toggle_onclick={toggle_onclick.clone()} remove_onclick={remove_onclick.clone()} toggle_edit_onclick={toggle_edit_onclick.clone()} edit={edit.clone()} />
                        }) }
                    </ul>
                </section>
                <footer class={classes!("footer", hidden_class)}>
                    <span class="todo-count">
                        <strong>{ total }</strong>
                        { " item(s) left" }
                    </span>
                    <ul class="filters">
                        { for Filter::iter().map(|flt| {
                            html! {
                                <FilterItem filter={flt} current_filter={state.filter.clone()} set_filter_onclick={set_filter_onclick.clone()}/>
                            }
                        }) }
                    </ul>
                    <button class="clear-completed" onclick={clear_completed_onclick}>
                        { format!("Clear completed ({})", completed) }
                    </button>
                </footer>
            </section>
            <InfoFooter />
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
