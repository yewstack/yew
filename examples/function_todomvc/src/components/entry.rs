use crate::state::Entry as Item;
use web_sys::{HtmlInputElement, MouseEvent};
use yew::events::{FocusEvent, KeyboardEvent};
use yew::{function_component, html, use_state, Callback, Classes, Properties, TargetCast};

#[derive(PartialEq, Properties, Clone)]
pub struct EntryProps {
    pub entry: Item,
    pub toggle_onclick: Callback<usize>,
    pub toggle_edit_onclick: Callback<usize>,
    pub remove_onclick: Callback<usize>,
    pub edit: Callback<(usize, String)>,
}

#[function_component(Entry)]
pub fn entry(props: &EntryProps) -> Html {
    let id = props.entry.id;
    let mut class = Classes::from("todo");

    if props.entry.editing {
        class.push("editing");
    }

    if props.entry.completed {
        class.push("completed");
    }

    html! {
        <li {class}>
            <div class="view">
                <input
                    type="checkbox"
                    class="toggle"
                    checked={props.entry.completed}
                    onclick={props.toggle_onclick.reform(move |_| id)}
                />
                <label ondblclick={props.toggle_edit_onclick.reform(move |_| id)}>
                    { &props.entry.description }
                </label>
                <button class="destroy" onclick={props.remove_onclick.reform(move |_| id)} />
            </div>
            <EntryEdit entry={props.entry.clone()} edit={props.edit.clone()} />
        </li>
    }
}

#[derive(PartialEq, Properties, Clone)]
pub struct EntryEditProps {
    pub entry: Item,
    pub edit: Callback<(usize, String)>,
}

#[function_component(EntryEdit)]
pub fn entry_edit(props: &EntryEditProps) -> Html {
    let id = props.entry.id;
    let edit_value = use_state(|| props.entry.description.clone());

    let onblur = {
        let edit = props.edit.clone();
        let edit_value = edit_value.clone();

        move |e: FocusEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();

            input.set_value("");
            edit_value.set(value.clone());
            edit.emit((id, value))
        }
    };

    let onkeypress = {
        let edit = props.edit.clone();
        let edit_value = edit_value.clone();

        move |e: KeyboardEvent| {
            (e.key() == "Enter").then(|| {
                let input: HtmlInputElement = e.target_unchecked_into();
                let value = input.value();

                input.set_value("");
                edit_value.set(value.clone());
                edit.emit((id, value))
            });
        }
    };

    let onmouseover = |e: MouseEvent| {
        e.target_unchecked_into::<HtmlInputElement>()
            .focus()
            .unwrap_or_default();
    };

    if props.entry.editing {
        html! {
            <input
                class="edit"
                type="text"
                value={(*edit_value).clone()}
                {onmouseover}
                {onblur}
                {onkeypress}
            />
        }
    } else {
        html! { <input type="hidden" /> }
    }
}
