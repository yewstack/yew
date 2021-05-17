use crate::{service, Routable};
use yew::prelude::*;

/// Props for [`Link`]
#[derive(Properties, Clone, PartialEq)]
pub struct LinkProps<R: Routable + Clone> {
    /// CSS classes to add to the anchor element (optional).
    #[prop_or_default]
    pub classes: Classes,
    /// Route that will be pushed when the anchor is clicked.
    pub route: R,
    pub children: Children,
}

/// A wrapper around `<a>` tag to be used with [`Router`](crate::Router)
pub struct Link<R: Routable + Clone + PartialEq + 'static> {
    link: ComponentLink<Self>,
    props: LinkProps<R>,
}

pub enum Msg {
    OnClick,
}

impl<R: Routable + Clone + PartialEq + 'static> Component for Link<R> {
    type Message = Msg;
    type Properties = LinkProps<R>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OnClick => {
                service::push_route(self.props.route.clone());
                false
            }
        }
    }

    fn change(&mut self, mut props: Self::Properties) -> ShouldRender {
        std::mem::swap(&mut self.props, &mut props);
        props != self.props
    }

    fn view(&self) -> Html {
        html! {
            <a class=self.props.classes.clone() onclick=self.link.callback(|_| Msg::OnClick)>{ self.props.children.clone() }</a>
        }
    }
}
