use crate::header::{ListHeader, Props as HeaderProps};
use crate::item::{ListItem, Props as ItemProps};
use crate::{Hovered, WeakComponentLink};
use yew::html::{ChildrenRenderer, NodeRef};
use yew::prelude::*;
use yew::virtual_dom::{VChild, VComp};

#[derive(Clone, PartialEq)]
pub enum Variants {
    Item(<ListItem as Component>::Properties),
    Header(<ListHeader as Component>::Properties),
}

impl From<ItemProps> for Variants {
    fn from(props: ItemProps) -> Self {
        Variants::Item(props)
    }
}

impl From<HeaderProps> for Variants {
    fn from(props: HeaderProps) -> Self {
        Variants::Header(props)
    }
}

#[derive(PartialEq, Clone)]
pub struct ListVariant {
    props: Variants,
}

impl<CHILD> From<VChild<CHILD>> for ListVariant
where
    CHILD: Component,
    CHILD::Properties: Into<Variants>,
{
    fn from(vchild: VChild<CHILD>) -> Self {
        Self {
            props: vchild.props.into(),
        }
    }
}

impl Into<Html> for ListVariant {
    fn into(self) -> Html {
        match self.props {
            Variants::Header(props) => {
                VComp::new::<ListHeader>(props, NodeRef::default(), None).into()
            }
            Variants::Item(props) => VComp::new::<ListItem>(props, NodeRef::default(), None).into(),
        }
    }
}

pub enum Msg {
    HeaderClick,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub children: ChildrenRenderer<ListVariant>,
    pub on_hover: Callback<Hovered>,
    pub weak_link: WeakComponentLink<List>,
}

pub struct List {
    props: Props,
    inactive: bool,
}

impl Component for List {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        props.weak_link.borrow_mut().replace(link);
        Self {
            props,
            inactive: false,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::HeaderClick => {
                self.inactive = !self.inactive;
                true
            }
        }
    }

    fn view(&self) -> Html {
        let inactive = if self.inactive { "inactive" } else { "" };
        let onmouseover = self.props.on_hover.reform(|_| Hovered::List);
        let onmouseout = self.props.on_hover.reform(|_| Hovered::None);
        html! {
            <div class="list-container" onmouseout=onmouseout onmouseover=onmouseover>
                <div class=classes!("list", inactive)>
                    { self.view_header() }
                    <div class="items">
                        { self.view_items() }
                    </div>
                </div>
            </div>
        }
    }
}

impl List {
    fn view_header(&self) -> Html {
        html! { for self.props.children.iter().filter(|c| matches!(c.props, Variants::Header(_))) }
    }

    fn view_items(&self) -> Html {
        self.props
            .children
            .iter()
            .filter(|c| matches!(&c.props, Variants::Item(props) if !props.hide))
            .enumerate()
            .map(|(i, mut c)| {
                if let Variants::Item(ref mut props) = c.props {
                    props.name = format!("#{} - {}", i + 1, props.name);
                }
                c
            })
            .collect::<Html>()
    }
}
