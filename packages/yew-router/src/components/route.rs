use yew::prelude::*;

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct RouteProps {
    pub to: String,
    pub children: Children,
}

pub struct Route {
    props: RouteProps,
}

impl Component for Route {
    type Message = ();
    type Properties = RouteProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, mut props: Self::Properties) -> ShouldRender {
        std::mem::swap(&mut self.props, &mut props);
        props != self.props
    }

    fn view(&self) -> Html {
        html! { for self.props.children.clone() }
    }
}
