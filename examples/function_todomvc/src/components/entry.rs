use web_sys::{HtmlInputElement, MouseEvent};
use yew::events::{Event, FocusEvent, KeyboardEvent};
use yew::prelude::*;

use crate::hooks::use_bool_toggle::use_bool_toggle;
use crate::state::Entry as Item;

#[derive(PartialEq, Properties, Clone)]
pub struct EntryProps {
    pub entry: Item,
    pub ontoggle: Callback<usize>,
    pub onremove: Callback<usize>,
    pub onedit: Callback<(usize, String)>,
}

#[function_component(Entry)]
pub fn entry(props: &EntryProps) -> Html {
    let id = props.entry.id;
    let mut class = Classes::from("todo");

    // We use the `use_bool_toggle` hook and set the default value to `false`
    // as the default we are not editing the the entry. When we want to edit the
    // entry we can call the toggle method on the `UseBoolToggleHandle`
    // which will trigger a re-render with the toggle value being `true` for that
    // render and after that render the value of toggle will be flipped back to
    // its default (`false`).
    // We are relying on the behavior of `onblur` and `onkeypress` to cause
    // another render so that this component will render again with the
    // default value of toggle.
    let edit_toggle = use_bool_toggle(false);
    let is_editing = *edit_toggle;

    if is_editing {
        class.push("editing");
    }

    if props.entry.completed {
        class.push("completed");
    }

    let ontoggle = {
        let ontoggle = props.ontoggle.clone();
        move |_| ontoggle.emit(id)
    };

    let onremove = {
        let onremove = props.onremove.clone();
        move |_| onremove.emit(id)
    };

    html! {
        <li {class}>
            <div class="view">
                <input
                    type="checkbox"
                    class="toggle"
                    checked={props.entry.completed}
                    onclick={ontoggle}
                />
                <label ondblclick={move |_| edit_toggle.clone().toggle()}>
                    { &props.entry.description }
                </label>
                <button class="destroy" onclick={onremove} />
            </div>
            <EntryEdit entry={props.entry.clone()} onedit={props.onedit.clone()} editing={is_editing} />
        </li>
    }
}

#[derive(PartialEq, Properties, Clone)]
pub struct EntryEditProps {
    pub entry: Item,
    pub onedit: Callback<(usize, String)>,
    pub editing: bool,
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

    if props.editing {
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
