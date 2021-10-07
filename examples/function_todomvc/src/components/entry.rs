use crate::state::Entry as Item;
use web_sys::{HtmlInputElement, MouseEvent};
use yew::events::{Event, FocusEvent, KeyboardEvent};
use yew::{function_component, html, Callback, Classes, Properties, TargetCast};

#[derive(PartialEq, Properties, Clone)]
pub struct EntryProps {
    pub entry: Item,
    pub ontoggle: Callback<usize>,
    pub ontoggle_edit: Callback<usize>,
    pub onremove: Callback<usize>,
    pub onedit: Callback<(usize, String)>,
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
                    onclick={props.ontoggle.reform(move |_| id)}
                />
                <label ondblclick={props.ontoggle_edit.reform(move |_| id)}>
                    { &props.entry.description }
                </label>
                <button class="destroy" onclick={props.onremove.reform(move |_| id)} />
            </div>
            <EntryEdit entry={props.entry.clone()} onedit={props.onedit.clone()} />
        </li>
    }
}

#[derive(PartialEq, Properties, Clone)]
pub struct EntryEditProps {
    pub entry: Item,
    pub onedit: Callback<(usize, String)>,
}

#[function_component(EntryEdit)]
pub fn entry_edit(props: &EntryEditProps) -> Html {
    let id = props.entry.id;

    let target_input_value = |e: &Event| {
        let input: HtmlInputElement = e.target_unchecked_into();
        input.value()
    };

    let onblur = {
        let edit = props.onedit.clone();

        move |e: FocusEvent| {
            let value = target_input_value(&e);
            edit.emit((id, value))
        }
    };

    let onkeypress = {
        let edit = props.onedit.clone();

        move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                let value = target_input_value(&e);
                edit.emit((id, value))
            }
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
                value={props.entry.description.clone()}
                {onmouseover}
                {onblur}
                {onkeypress}
            />
        }
    } else {
        html! { <input type="hidden" /> }
    }
}
