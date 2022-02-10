//! Primitive Components & Properties Types

use std::marker::PhantomData;

use crate::html;
use crate::html::{BaseComponent, Children, Component, Context, Html, Properties};

/// A [Properties] type with Children being the only property.
#[derive(Debug, Properties, PartialEq)]
pub struct ChildrenProps {
    #[prop_or_default]
    pub children: Children,
}

/// A Component to represent another component that does not exist in current implementation.
///
/// During Hydration, Yew expected the Virtual DOM hierarchy to be the match the the layout used in server-side
/// renering. However, sometimes it is possible / reasonable to omit certain components from one
/// side of the implementation. This component is used to represent a component as if a component "existed"
/// in the place it is defined.
///
/// # Warning
///
/// The Real DOM hierarchy must also match the server-side rendered artifact. This component is
/// only usable when the original component does not introduce any additional elements. (e.g.: Context
/// Providers)
///
/// A generic parameter is provided to help identify the component to be substituted. This is not
/// enforced.
#[derive(Debug)]
pub struct PhantomComponent<T>
where
    T: BaseComponent,
{
    _marker: PhantomData<T>,
}

impl<T> Component for PhantomComponent<T>
where
    T: BaseComponent,
{
    type Properties = ChildrenProps;
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            _marker: PhantomData,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let children = ctx.props().children.clone();
        html! { <>{children}</> }
    }
}
