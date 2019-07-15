//! This module contains implementation of `Select` component.
//! You can use it instead `<select>` tag, because the component
//! helps you to track selected value in an original type. Example:
//!
//! ```
//! enum Scene {
//!     First,
//!     Second,
//! }
//!
//! fn view() -> Html<Model> {
//!     let scenes = vec![Scene::First, Scene::Second];
//!     html! {
//!         <Select<Scenes> options=scenes />
//!     }
//! }

use crate::callback::Callback;
use crate::html;
use crate::html::{ChangeData, Component, ComponentLink, Html, Renderable, ShouldRender};

/// `Select` component.
pub struct Select<T> {
    props: Props<T>,
}

/// Internal message of the component.
pub enum Msg {
    /// This message indicates the option with id selected.
    Selected(Option<usize>),
}

/// Properties of `Select` component.
#[derive(PartialEq, Clone)]
pub struct Props<T> {
    /// Initially selected value.
    pub selected: Option<T>,
    /// Disabled the component's selector.
    pub disabled: bool,
    /// Options are available to choose.
    pub options: Vec<T>,
    /// Callback to handle changes.
    pub onchange: Option<Callback<T>>,
}

impl<T> Default for Props<T> {
    fn default() -> Self {
        Props {
            selected: None,
            disabled: false,
            options: Vec::new(),
            onchange: None,
        }
    }
}

impl<T> Component for Select<T>
where
    T: PartialEq + Clone + 'static,
{
    type Message = Msg;
    type Properties = Props<T>;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Selected(value) => {
                if let Some(idx) = value {
                    if let Some(ref mut callback) = self.props.onchange {
                        let item = self.props.options.get(idx - 1).cloned();
                        if let Some(value) = item {
                            callback.emit(value);
                        }
                    }
                }
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }
}

impl<T> Renderable<Select<T>> for Select<T>
where
    T: ToString + PartialEq + Clone + 'static,
{
    fn view(&self) -> Html<Self> {
        let selected = self.props.selected.as_ref();
        let view_option = |value: &T| {
            let flag = selected == Some(value);
            html! {
                <option selected=flag>{ value.to_string() }</option>
            }
        };
        html! {
            <select disabled=self.props.disabled
                    onchange=|event| {
                        match event {
                            ChangeData::Select(elem) => {
                                let value = elem.selected_index().map(|x| x as usize);
                                Msg::Selected(value)
                            }
                            _ => {
                                unreachable!();
                            }
                        }
                    }>
                <option disabled=true selected=selected.is_none()>
                    { "↪" }
                </option>
                { for self.props.options.iter().map(view_option) }
            </select>
        }
    }
}
