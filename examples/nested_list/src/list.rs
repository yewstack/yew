use crate::header::{ListHeader, Props as HeaderProps};
use crate::item::{ListItem, Props as ItemProps};
use crate::{Hovered, WeakContextRef};
use std::{cell::RefCell, rc::Rc};
use yew::html::{ChildrenRenderer, NodeRef};
use yew::prelude::*;
use yew::virtual_dom::{VChild, VComp};

#[derive(Clone, PartialEq)]
pub enum Variants {
    Item(RefCell<<ListItem as Component>::Properties>),
    Header(RefCell<<ListHeader as Component>::Properties>),
}

impl From<RefCell<ItemProps>> for Variants {
    fn from(props: RefCell<ItemProps>) -> Self {
        Variants::Item(props)
    }
}

impl From<RefCell<HeaderProps>> for Variants {
    fn from(props: RefCell<HeaderProps>) -> Self {
        Variants::Header(props)
    }
}

#[derive(Clone, PartialEq)]
pub struct ListVariant {
    props: Variants,
}

impl<CHILD> From<VChild<CHILD>> for ListVariant
where
    CHILD: Component,
    RefCell<CHILD::Properties>: Into<Variants>,
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
                VComp::new::<ListHeader>(Rc::new(props.into_inner()), NodeRef::default(), None)
                    .into()
            }
            Variants::Item(props) => {
                VComp::new::<ListItem>(Rc::new(props.into_inner()), NodeRef::default(), None).into()
            }
        }
    }
}

pub enum Msg {
    HeaderClick,
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub children: ChildrenRenderer<ListVariant>,
    pub on_hover: Callback<Hovered>,
    pub weak_link: WeakContextRef<List>,
}

pub struct List {
    inactive: bool,
}

impl Component for List {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.props.weak_link.borrow_mut().replace(ctx.clone());
        Self { inactive: false }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::HeaderClick => {
                self.inactive = !self.inactive;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let inactive = if self.inactive { "inactive" } else { "" };
        let onmouseover = ctx.props.on_hover.reform(|_| Hovered::List);
        let onmouseout = ctx.props.on_hover.reform(|_| Hovered::None);
        html! {
            <div class="list-container" onmouseout=onmouseout onmouseover=onmouseover>
                <div class=("list", inactive)>
                    { self.view_header(ctx) }
                    <div class="items">
                        { self.view_items(ctx) }
                    </div>
                </div>
            </div>
        }
    }
}

impl List {
    fn view_header(&self, ctx: &Context<Self>) -> Html {
        html! { for ctx.props.children.iter().filter(|c| matches!(c.props, Variants::Header(_))) }
    }

    fn view_items(&self, ctx: &Context<Self>) -> Html {
        ctx.props
            .children
            .iter()
            .filter(|c| matches!(&c.props, Variants::Item(props) if !props.borrow().hide))
            .enumerate()
            .map(|(i, mut c)| {
                if let Variants::Item(ref mut props) = c.props {
                    props.borrow_mut().name = format!("#{} - {}", i + 1, props.borrow().name);
                }
                c
            })
            .collect::<Html>()
    }
}
