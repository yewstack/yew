use super::{Hovered, WeakComponentLink};
use crate::{header::ListHeader, header::Props as HeaderProps};
use crate::{item::ListItem, item::Props as ItemProps};
use yew::html::{ChildrenRenderer, NodeRef};
use yew::prelude::*;
use yew::virtual_dom::{VChild, VComp, VNode};

#[derive(Clone)]
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

#[derive(Clone)]
pub struct ListVariant {
    props: Variants,
}

#[derive(Clone, Properties)]
pub struct Props {
    pub children: ChildrenRenderer<ListVariant>,
    pub on_hover: Callback<Hovered>,
    pub weak_link: WeakComponentLink<List>,
}

pub struct List {
    props: Props,
    inactive: bool,
}

pub enum Msg {
    HeaderClick,
}

impl Component for List {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        *props.weak_link.borrow_mut() = Some(link);
        List {
            props,
            inactive: false,
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
                <div class=vec!["list", inactive]>
                    {self.view_header()}
                    <div class="items">
                        {self.view_items()}
                    </div>
                </div>
            </div>
        }
    }
}

impl List {
    fn view_header(&self) -> Html {
        html! {{
            for self.props.children.iter().filter(|c| match c.props {
                Variants::Header(_) => true,
                _ => false
            })
        }}
    }

    fn view_items(&self) -> Html {
        html! {{
            for self.props.children.iter().filter(|c| match &c.props {
                Variants::Item(props) => !props.hide,
                _ => false,
            }).enumerate().map(|(i, mut c)| {
                if let Variants::Item(ref mut props) = c.props {
                    props.name = format!("#{} - {}", i + 1, props.name);
                }
                c
            })
        }}
    }
}

impl<CHILD> From<VChild<CHILD>> for ListVariant
where
    CHILD: Component,
    CHILD::Properties: Into<Variants>,
{
    fn from(vchild: VChild<CHILD>) -> Self {
        ListVariant {
            props: vchild.props.into(),
        }
    }
}

impl Into<VNode> for ListVariant {
    fn into(self) -> VNode {
        match self.props {
            Variants::Header(props) => VComp::new::<ListHeader>(props, NodeRef::default()).into(),
            Variants::Item(props) => VComp::new::<ListItem>(props, NodeRef::default()).into(),
        }
    }
}
