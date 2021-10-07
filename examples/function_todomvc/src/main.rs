use gloo::storage::{LocalStorage, Storage};
use state::{Entry, Filter, State};
use std::rc::Rc;
use strum::IntoEnumIterator;
use yew::{classes, function_component, html, use_effect_with_deps, use_reducer, Callback};

mod components;
mod hooks;
mod state;

use components::{
    entry::Entry as EntryItem, filter::Filter as FilterItem, header_input::HeaderInput,
    info_footer::InfoFooter,
};

pub enum Action {
    Add(String),
    Edit((usize, String)),
    Remove(usize),
    SetFilter(Filter),
    ToggleAll,
    Toggle(usize),
    ClearCompleted,
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
        move |state| {
            LocalStorage::set(KEY, &state.clone().entries).expect("failed to set");
            || ()
        },
        state.clone(),
    );

    // Callbacks
    let onremove = {
        let state = state.clone();
        Callback::from(move |id: usize| state.dispatch(Action::Remove(id)))
    };

    let ontoggle = {
        let state = state.clone();
        Callback::from(move |id: usize| state.dispatch(Action::Toggle(id)))
    };

    let ontoggle_all = {
        let state = state.clone();
        Callback::from(move |_| state.dispatch(Action::ToggleAll))
    };

    let onclear_completed = {
        let state = state.clone();
        Callback::from(move |_| state.dispatch(Action::ClearCompleted))
    };

    let onedit = {
        let state = state.clone();
        Callback::from(move |(id, value): (usize, String)| {
            state.dispatch(Action::Edit((id, value)));
        })
    };

    let onadd = {
        let state = state.clone();
        Callback::from(move |value: String| {
            state.dispatch(Action::Add(value));
        })
    };

    let onset_filter = {
        let state = state.clone();
        Callback::from(move |filter: Filter| {
            state.dispatch(Action::SetFilter(filter));
        })
    };

    // Helpers
    let completed = state
        .entries
        .iter()
        .filter(|entry| Filter::Completed.fits(entry))
        .count();

    let is_all_completed = state
        .entries
        .iter()
        .all(|e| state.filter.fits(e) & e.completed);

    let total = state.entries.len();

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
                    <HeaderInput {onadd} />
                </header>
                <section class={classes!("main", hidden_class)}>
                    <input
                        type="checkbox"
                        class="toggle-all"
                        id="toggle-all"
                        checked={is_all_completed}
                        onclick={ontoggle_all}
                    />
                    <label for="toggle-all" />
                    <ul class="todo-list">
                        { for state.entries.iter().filter(|e| state.filter.fits(e)).cloned().map(|entry|
                            html! {
                                <EntryItem {entry}
                                    ontoggle={ontoggle.clone()}
                                    onremove={onremove.clone()}
                                    onedit={onedit.clone()}
                                />
                        }) }
                    </ul>
                </section>
                <footer class={classes!("footer", hidden_class)}>
                    <span class="todo-count">
                        <strong>{ total }</strong>
                        { " item(s) left" }
                    </span>
                    <ul class="filters">
                        { for Filter::iter().map(|filter| {
                            html! {
                                <FilterItem {filter}
                                    selected={state.filter == filter}
                                    onset_filter={onset_filter.clone()}
                                />
                            }
                        }) }
                    </ul>
                    <button class="clear-completed" onclick={onclear_completed}>
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
