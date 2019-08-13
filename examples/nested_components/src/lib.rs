use yew::prelude::*;

mod child;
mod parent;

use child::Child;
use parent::{Msg as ParentMsg, Parent};

pub struct Model {
    child_name: String,
}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            child_name: "Bobby".to_owned(),
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let child_name = self.child_name.clone();
        html! {
            <Parent>
                <Child name=&child_name on_click=|_| ParentMsg::ChildClick />
            </Parent>
        }
    }
}
