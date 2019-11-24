use super::Hovered;
use yew::html::Children;
use yew::prelude::*;

pub struct ListItem {
    link: ComponentLink<Self>,
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

pub enum Msg {
    Hover,
}

impl Component for ListItem {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ListItem { link, props }
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

    fn view(&self) -> Html {
        let onmouseover = self.link.send_back(|_| Msg::Hover);
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
