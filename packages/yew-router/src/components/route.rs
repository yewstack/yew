use yew::prelude::*;

/// Props for [`Route`].
#[derive(Properties, Clone, PartialEq, Debug)]
pub struct RouteProps {
    /// The path for this route. Dynamic segments are specified by prefixing them with `:`.
    /// **Example**: `/path/:value`
    pub to: String,
    /// The elements displayed on this route.
    pub children: Children,
}

/// Specifies a route for the [`Router`](crate::Router).
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
