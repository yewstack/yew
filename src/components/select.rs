//! This module contains implementation of `Select` component.
//! You can use it instead `<select>` tag, because the component
//! helps you to track selected value in an original type. Example:
//!
//! ```
//!# use yew::{Html, Component, components::Select, ComponentLink, html};
//! #[derive(PartialEq, Clone)]
//! enum Scene {
//!     First,
//!     Second,
//! }
//!# struct Model { link: ComponentLink<Self> };
//!# impl Component for Model {
//!#     type Message = ();type Properties = ();
//!#     fn create(props: Self::Properties,link: ComponentLink<Self>) -> Self {unimplemented!()}
//!#     fn update(&mut self,msg: Self::Message) -> bool {unimplemented!()}
//!#     fn change(&mut self, _: Self::Properties) -> bool {unimplemented!()}
//!#     fn view(&self) -> Html {unimplemented!()}}
//! impl ToString for Scene {
//!     fn to_string(&self) -> String {
//!         match self {
//!             Scene::First => "First".to_string(),
//!             Scene::Second => "Second".to_string()
//!         }
//!     }
//! }
//!
//! fn view(link: ComponentLink<Model>) -> Html {
//!     let scenes = vec![Scene::First, Scene::Second];
//!     html! {
//!         <Select<Scene> options=scenes onchange=link.callback(|_| ()) />
//!     }
//! }
//! ```

use crate::callback::Callback;
use crate::html::{ChangeData, Component, ComponentLink, Html, NodeRef, ShouldRender};
use crate::macros::{html, Properties};
use cfg_if::cfg_if;
use cfg_match::cfg_match;
cfg_if! {
    if #[cfg(feature = "std_web")] {
        use stdweb::web::html_element::SelectElement;
    } else if #[cfg(feature = "web_sys")] {
        use web_sys::HtmlSelectElement as SelectElement;
    }
}

/// `Select` component.
#[derive(Debug)]
pub struct Select<T: ToString + PartialEq + Clone + 'static> {
    props: Props<T>,
    select_ref: NodeRef,
    link: ComponentLink<Self>,
}

/// Internal message of the component.
#[derive(Debug)]
pub enum Msg {
    /// This message indicates the option with id selected.
    Selected(Option<usize>),
}

/// Properties of `Select` component.
#[derive(PartialEq, Clone, Properties, Debug)]
pub struct Props<T: Clone> {
    /// Initially selected value.
    #[prop_or_default]
    pub selected: Option<T>,
    /// Disabled the component's selector.
    #[prop_or_default]
    pub disabled: bool,
    /// Options are available to choose.
    #[prop_or_default]
    pub options: Vec<T>,
    /// Callback to handle changes.
    pub onchange: Callback<T>,
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
                    let item = self.props.options.get(idx - 1).cloned();
                    if let Some(value) = item {
                        self.props.onchange.emit(value);
                    }
                }
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.selected != props.selected {
            if let Some(select) = self.select_ref.cast::<SelectElement>() {
                let val = props
                    .selected
                    .as_ref()
                    .map(|v| v.to_string())
                    .unwrap_or_default();
                cfg_match! {
                    feature = "std_web" => select.set_raw_value(&val),
                    feature = "web_sys" => select.set_value(&val),
                }
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
            <select ref=self.select_ref.clone() disabled=self.props.disabled onchange=self.onchange()>
                <option value="" disabled=true selected=selected.is_none()>
                    { "↪" }
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
    fn onchange(&self) -> Callback<ChangeData> {
        self.link.callback(|event| match event {
            ChangeData::Select(elem) => {
                let value = elem.selected_index();
                let value = cfg_match! {
                    feature = "std_web" => value.map(|x| x as usize),
                    feature = "web_sys" => Some(value as usize),
                };
                Msg::Selected(value)
            }
            _ => {
                unreachable!();
            }
        })
    }
}
