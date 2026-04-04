use gloo::storage::{LocalStorage, Storage};
use state::{Action, Filter, State};
use strum::IntoEnumIterator;
use yew::prelude::*;

mod components;
mod hooks;
mod state;

use components::entry::Entry as EntryItem;
use components::filter::Filter as FilterItem;
use components::header_input::HeaderInput;
use components::info_footer::InfoFooter;

const KEY: &str = "yew.functiontodomvc.self";

#[function_component(App)]
fn app() -> Html {
    let state = use_reducer(|| State {
        entries: LocalStorage::get(KEY).unwrap_or_else(|_| vec![]),
        filter: Filter::All, // TODO: get from uri
    });

    // Effect
    use_effect_with(state.clone(), |state| {
        LocalStorage::set(KEY, &state.clone().entries).expect("failed to set");
    });

    // Callbacks
    fn make_callback<E, F>(state: &UseReducerHandle<State>, f: F) -> Callback<E>
    where
        F: Fn(E) -> Action + 'static,
    {
        let state = state.clone();
        Callback::from(move |e: E| state.dispatch(f(e)))
    }

    let onremove = make_callback(&state, Action::Remove);
    let ontoggle = make_callback(&state, Action::Toggle);
    let ontoggle_all = make_callback(&state, |_| Action::ToggleAll);
    let onclear_completed = make_callback(&state, |_| Action::ClearCompleted);
    let onedit = make_callback(&state, Action::Edit);
    let onadd = make_callback(&state, Action::Add);
    let onset_filter = make_callback(&state, Action::SetFilter);

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
                                    ontoggle={&ontoggle}
                                    onremove={&onremove}
                                    onedit={&onedit}
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
                                    onset_filter={&onset_filter}
                                />
                            }
                        }) }
                    </ul>
                    <button class="clear-completed" onclick={onclear_completed}>
                        { format!("Clear completed ({completed})") }
                    </button>
                </footer>
            </section>
            <InfoFooter />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
