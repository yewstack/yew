use crate::RouterService;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct LinkProps {
    #[prop_or_default]
    pub classes: String,
    pub route: String,
    pub children: Children,
}

pub struct Link {
    link: ComponentLink<Self>,
    props: LinkProps,
}

pub enum Msg {
    OnClick,
}

impl Component for Link {
    type Message = Msg;
    type Properties = LinkProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OnClick => {
                let route = self.props.route.clone();
                RouterService::push(&route);
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
