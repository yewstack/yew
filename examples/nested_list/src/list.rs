use std::rc::Rc;

use yew::html::ChildrenRenderer;
use yew::prelude::*;
use yew::virtual_dom::{VChild, VComp};

use crate::header::{ListHeader, Props as HeaderProps};
use crate::item::{ListItem, Props as ItemProps};
use crate::{Hovered, WeakComponentLink};

#[derive(Clone, PartialEq)]
pub enum Variants {
    Item(Rc<<ListItem as Component>::Properties>),
    Header(Rc<<ListHeader as Component>::Properties>),
}

impl From<ItemProps> for Variants {
    fn from(props: ItemProps) -> Self {
        Variants::Item(Rc::new(props))
    }
}

impl From<HeaderProps> for Variants {
    fn from(props: HeaderProps) -> Self {
        Variants::Header(Rc::new(props))
    }
}

#[derive(PartialEq, Clone)]
pub struct ListVariant {
    props: Variants,
}

impl<CHILD> From<VChild<CHILD>> for ListVariant
where
    CHILD: Component,
    CHILD::Properties: Into<Variants> + Clone,
{
    fn from(vchild: VChild<CHILD>) -> Self {
        Self {
            props: (*vchild.props).clone().into(),
        }
    }
}

impl From<ListVariant> for Html {
    fn from(variant: ListVariant) -> Html {
        match variant.props {
            Variants::Header(props) => VComp::new::<ListHeader>(props, None).into(),
            Variants::Item(props) => VComp::new::<ListItem>(props, None).into(),
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
    inactive: bool,
}

impl Component for List {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.props()
            .weak_link
            .borrow_mut()
            .replace(ctx.link().clone());
        Self { inactive: false }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HeaderClick => {
                self.inactive = !self.inactive;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let inactive = if self.inactive { "inactive" } else { "" };
        let onmouseover = ctx.props().on_hover.reform(|e: MouseEvent| {
            e.stop_propagation();
            Hovered::List
        });
        html! {
            <div class="list-container" {onmouseover}>
                <div class={classes!("list", inactive)}>
                    { Self::view_header(&ctx.props().children) }
                    <div class="items">
                        { Self::view_items(&ctx.props().children) }
                    </div>
                </div>
            </div>
        }
    }
}

impl List {
    fn view_header(children: &ChildrenRenderer<ListVariant>) -> Html {
        html! { for children.iter().filter(|c| matches!(c.props, Variants::Header(_))) }
    }

    fn view_items(children: &ChildrenRenderer<ListVariant>) -> Html {
        children
            .iter()
            .filter(|c| matches!(&c.props, Variants::Item(props) if !props.hide))
            .enumerate()
            .map(|(i, mut c)| {
                if let Variants::Item(props) = c.props {
                    let mut props = (*props).clone();
                    props.name = format!("#{} - {}", i + 1, props.name);
                    c.props = Variants::Item(Rc::new(props));
                }
                c
            })
            .collect::<Html>()
    }
}
