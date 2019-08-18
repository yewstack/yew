use crate::child::Child;
use yew::prelude::*;
use yew::virtual_dom::VChild;

pub enum Msg {
    Click,
    ChildClick,
}

type Children<T> = Box<dyn Fn() -> Vec<VChild<T, Parent>>>;

#[derive(Properties)]
pub struct Props {
    #[props(required)]
    pub children: Children<Child>,
}

pub struct Parent {
    props: Props,
    clicker: String,
}

impl Component for Parent {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Parent {
            clicker: "none".to_owned(),
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => self.clicker = "self".to_string(),
            Msg::ChildClick => self.clicker = "child".to_string(),
        }
        true
    }
}

impl Renderable<Parent> for Parent {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="parent">
                { format!("Last clicked by {}", self.clicker) }
                <button onclick=|_| Msg::Click>{"Parent button"}</button>
                { for (self.props.children)().into_iter().filter(|c| !c.props.hide) }
            </div>
        }
    }
}
