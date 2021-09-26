use crate::state::Entry as Item;
use web_sys::HtmlInputElement;
use yew::events::{FocusEvent, KeyboardEvent};
use yew::{
    function_component, html, use_state, /* use_ref, */ Callback, Classes, Properties,
    TargetCast, /* NodeRef */
};

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
    let mut class = Classes::from("todo");
    if props.entry.editing {
        class.push(" editing");
    }
    if props.entry.completed {
        class.push(" completed");
    }
    html! {
        <li {class}>
        <div class="view">
            <input
                type="checkbox"
                class="toggle"
                checked={props.entry.completed}
                onclick={
                    let props = props.clone();
                    move |_| props.toggle_onclick.emit(props.entry.id)}
            />
            <label ondblclick={
                let props = props.clone();
                move |_| props.toggle_edit_onclick.emit(props.entry.id)}>{ &props.entry.description }</label>
            <button class="destroy" onclick={
                let props = props.clone();
                move |_| props.remove_onclick.emit(props.entry.id)} />
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
    let init_placeholder = props.entry.description.clone();
    let edit_value = use_state(|| init_placeholder.clone());
    // let focus_ref = use_ref(|| NodeRef::default()); Focus ref is not working

    let onblur = {
        let props = props.clone();
        let edit_value = edit_value.clone();
        move |e: FocusEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            input.set_value("");
            edit_value.set(value.to_owned());
            props.edit.emit((props.entry.id, value))
        }
    };

    let onkeypress = {
        let props = props.clone();
        let edit_value = edit_value.clone();
        move |e: KeyboardEvent| {
            (e.key() == "Enter").then(|| {
                let input: HtmlInputElement = e.target_unchecked_into();
                let value = input.value();
                input.set_value("");
                edit_value.set(value.to_owned());
                props.edit.emit((props.entry.id, value))
            });
        }
    };

    if props.entry.editing {
        html! {
            <input
                class="edit"
                type="text"
                // ref={focus_ref}
                value={(*edit_value).clone()}
                // onmouseover={link.callback(|_| Msg::Focus)}
                {onblur}
                {onkeypress}
            />
        }
    } else {
        html! { <input type="hidden" /> }
    }
}
