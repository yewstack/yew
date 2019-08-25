use crate::list::Hovered;
use yew::html::Children;
use yew::prelude::*;

pub struct ListItem {
    props: Props,
}

#[derive(Properties)]
// #[props(ListItem)]
pub struct Props {
    pub hide: bool,
    #[props(required)]
    pub on_hover: Callback<Hovered>,
    #[props(required)]
    pub name: String,
    pub children: Children<ListItem>,
}

pub enum Msg {
    Hover,
}

impl Component for ListItem {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        ListItem { props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Hover => {
                self.props
                    .on_hover
                    .emit(Hovered::Item(self.props.name.clone()));
            }
        }
        false
    }
}

impl Renderable<ListItem> for ListItem {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="list-item" onmouseover=|_| Msg::Hover>
                { &self.props.name }
                { self.view_details() }
            </div>
        }
    }
}

impl ListItem {
    fn view_details(&self) -> Html<Self> {
        let children = self.props.children.to_vec();
        if children.is_empty() {
            return html! {};
        }

        html! {
            <div class="list-item-details">
                { for children.into_iter() }
            </div>
        }
    }
}
