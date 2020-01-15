use crate::Hovered;
use yew::html::Children;
use yew::prelude::*;

pub struct ListItem {
    props: Props,
}

#[derive(Clone, Properties)]
pub struct Props {
    pub hide: bool,
    #[props(required)]
    pub on_hover: Callback<Hovered>,
    #[props(required)]
    pub name: String,
    pub children: Children,
}

impl Component for ListItem {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        ListItem { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let name = self.props.name.clone();
        let onmouseover = self
            .props
            .on_hover
            .reform(move |_| Hovered::Item(name.clone()));
        html! {
            <div class="list-item" onmouseover=onmouseover>
                { &self.props.name }
                { self.view_details() }
            </div>
        }
    }
}

impl ListItem {
    fn view_details(&self) -> Html {
        if self.props.children.is_empty() {
            return html! {};
        }

        html! {
            <div class="list-item-details">
                { self.props.children.render() }
            </div>
        }
    }
}
