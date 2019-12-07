use crate::{header::Props as HeaderProps, ListHeader};
use crate::{item::Props as ItemProps, ListItem};
use crate::Hovered;
use yew::html::{ChildrenRenderer, NodeRef};
use yew::prelude::*;
use yew::virtual_dom::{VChild, VComp, VNode};

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

pub struct ListVariant {
    props: Variants,
}

#[derive(Properties)]
pub struct Props {
    #[props(required)]
    pub children: ChildrenRenderer<ListVariant>,
    #[props(required)]
    pub on_hover: Callback<Hovered>,
}

pub struct List {
    props: Props,
}

impl Component for List {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        List { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let onmouseover = self.props.on_hover.reform(|_| Hovered::List);
        let onmouseout = self.props.on_hover.reform(|_| Hovered::None);
        html! {
            <div class="list-container" onmouseout=onmouseout onmouseover=onmouseover>
                <div class="list">
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
