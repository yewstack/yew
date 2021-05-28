use std::rc::Rc;

use yew::{html, Callback, Children, Component, ComponentLink, ContextProvider, Html, Properties};

use crate::{context::RoutingContext, Routable, RouterAction};

/// Props for [`ContextProvider`]
#[derive(Debug, Clone, PartialEq, Properties)]
pub struct MountProps<T: Routable> {
    /// Context value to be passed down
    pub route: Rc<T>,
    pub onroute: Callback<RouterAction<T>>,
    /// Children
    pub children: Children,
}

/// The context provider component.
///
/// Every child (direct or indirect) of this component may access the context value.
/// In order to consume contexts, [`ComponentLink::context`][Scope::context] method is used,
/// In function components the `use_context` hook is used.
#[derive(Debug)]
pub struct Mount<T: Routable> {
    context: RoutingContext<T>,
    children: Children,
}

impl<T: Routable> Component for Mount<T> {
    type Message = ();
    type Properties = MountProps<T>;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            children: props.children,
            context: RoutingContext {
                route: props.route,
                onroute: props.onroute,
            },
        }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.children = props.children;
        self.context = RoutingContext {
            route: props.route,
            onroute: props.onroute,
        };
        true
    }

    fn view(&self) -> Html {
        let context = self.context.clone();
        html! {
            <ContextProvider<RoutingContext<T>> context=context>
                { self.children.clone() }
            </ContextProvider<RoutingContext<T>>>
        }
    }
}
