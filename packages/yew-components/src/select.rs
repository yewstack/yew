//! This module contains the implementation of the `Select` component.

use web_sys::HtmlSelectElement;
use yew::callback::Callback;
use yew::html::{ChangeData, Component, ComponentLink, Html, NodeRef, ShouldRender};
use yew::{html, Properties};

/// An alternative to the HTML `<select>` tag.
///
/// The display of options is handled by the `ToString` implementation on their
/// type.
///
/// # Example
///
/// ```
///# use std::fmt;
///# use yew::{Html, Component, ComponentLink, html};
///# use yew_components::Select;
/// #[derive(PartialEq, Clone)]
/// enum Scene {
///     First,
///     Second,
/// }
///# struct Model { link: ComponentLink<Self> };
///# impl Component for Model {
///#     type Message = ();type Properties = ();
///#     fn create(props: Self::Properties,link: ComponentLink<Self>) -> Self {unimplemented!()}
///#     fn update(&mut self,msg: Self::Message) -> bool {unimplemented!()}
///#     fn change(&mut self, _: Self::Properties) -> bool {unimplemented!()}
///#     fn view(&self) -> Html {unimplemented!()}}
/// impl fmt::Display for Scene {
///     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
///         match self {
///             Scene::First => write!(f, "{}", "First"),
///             Scene::Second => write!(f, "{}", "Second"),
///         }
///     }
/// }
///
/// fn view(link: ComponentLink<Model>) -> Html {
///     let scenes = vec![Scene::First, Scene::Second];
///     html! {
///         <Select<Scene> options=scenes on_change=link.callback(|_| ()) />
///     }
/// }
/// ```
///
/// # Properties
///
/// Only the `on_change` property is mandatory. Other (optional) properties
/// are `selected`, `disabled`, `options`, `class`, `id`, and `placeholder`.
#[derive(Debug)]
pub struct Select<T: ToString + PartialEq + Clone + 'static> {
    props: Props<T>,
    select_ref: NodeRef,
    link: ComponentLink<Self>,
}

/// Messages sent internally as part of the select component
#[derive(Debug)]
pub enum Msg {
    /// Sent when the user selects a new option.
    Selected(Option<usize>),
}

/// Properties of the `Select` component.
#[derive(PartialEq, Clone, Properties, Debug)]
pub struct Props<T: Clone> {
    /// Initially selected value.
    #[prop_or_default]
    pub selected: Option<T>,
    /// Whether or not the selector should be disabled.
    #[prop_or_default]
    pub disabled: bool,
    /// A vector of options which the end user can choose from.
    #[prop_or_default]
    pub options: Vec<T>,
    /// Classes to be applied to the `<select>` tag
    #[prop_or_default]
    pub class: String,
    /// The ID for the `<select>` tag
    #[prop_or_default]
    pub id: String,
    /// Placeholder value, shown at the top as a disabled option
    #[prop_or(String::from("↪"))]
    pub placeholder: String,
    /// A callback which is called when the value of the `<select>` changes.
    pub on_change: Callback<T>,
}

impl<T> Component for Select<T>
where
    T: ToString + PartialEq + Clone + 'static,
{
    type Message = Msg;
    type Properties = Props<T>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            select_ref: NodeRef::default(),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Selected(value) => {
                if let Some(idx) = value {
                    let item = self.props.options.get(idx - 1);
                    if let Some(value) = item {
                        self.props.on_change.emit(value.clone());
                    }
                }
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.selected != props.selected {
            if let Some(select) = self.select_ref.cast::<HtmlSelectElement>() {
                let val = props
                    .selected
                    .as_ref()
                    .map(|v| v.to_string())
                    .unwrap_or_default();
                select.set_value(&val);
            }
        }
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let selected = self.props.selected.as_ref();
        let view_option = |value: &T| {
            let flag = selected == Some(value);
            html! {
                <option value=value.to_string() selected=flag>{ value.to_string() }</option>
            }
        };

        html! {
            <select
                ref=self.select_ref.clone()
                id=self.props.id
                class=self.props.class.clone()
                disabled=self.props.disabled
                onchange=self.on_change()
            >
                <option value="" disabled=true selected=selected.is_none()>
                    { self.props.placeholder.clone() }
                </option>
                { for self.props.options.iter().map(view_option) }
            </select>
        }
    }
}

impl<T> Select<T>
where
    T: ToString + PartialEq + Clone + 'static,
{
    fn on_change(&self) -> Callback<ChangeData> {
        self.link.callback(|event| match event {
            ChangeData::Select(elem) => {
                let value = elem.selected_index();
                Msg::Selected(Some(value as usize))
            }
            _ => unreachable!(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_select() {
        let on_change = Callback::<u8>::default();
        html! {
            <Select<u8> on_change=on_change />
        };
    }

    #[test]
    fn can_create_select_with_class() {
        let on_change = Callback::<u8>::default();
        html! {
            <Select<u8> on_change=on_change class="form-control" />
        };
    }

    #[test]
    fn can_create_select_with_id() {
        let on_change = Callback::<u8>::default();
        html! {
            <Select<u8> on_change=on_change id="test-select" />
        };
    }

    #[test]
    fn can_create_select_with_placeholder() {
        let on_change = Callback::<u8>::default();
        html! {
            <Select<u8> on_change=on_change placeholder="--Please choose an option--" />
        };
    }
}
