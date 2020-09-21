use crate::Hovered;
use yew::prelude::*;

#[derive(PartialEq, Clone, Properties)]
pub struct Props {
    #[prop_or_default]
    pub hide: bool,
    pub on_hover: Callback<Hovered>,
    pub name: String,
    #[prop_or_default]
    pub children: Children,
}

pub struct ListItem {
    props: Props,
}

impl Component for ListItem {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn view(&self) -> Html {
        let onmouseover = {
            let name = self.props.name.clone();
            self.props.on_hover.reform(move |e: MouseEvent| {
                e.stop_propagation();
                Hovered::Item(name.clone())
            })
        };
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
            html! {}
        } else {
            html! {
                <div class="list-item-details">
                    { self.props.children.clone() }
                </div>
            }
        }
    }
}
