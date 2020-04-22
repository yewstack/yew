use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yewtil::ptr::Lrc;

mod child;
use crate::child::Child;

pub struct Model {
    text: Lrc<String>,
}

pub enum Msg {
    UpdateTextAtADistance,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            text: Lrc::new("".to_string()),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateTextAtADistance => self.text.update(),
        }
    }

    fn view(&self) -> Html {
        html! {
            <>
                <div>
                   {&*self.text}
                </div>
                // Either of the children's update buttons will cause this component's text
                // to update to the most recently edited text.
                <div>
                    <Child text=&self.text callback = |_| Msg::UpdateTextAtADistance />
                </div>
                <div>
                    <Child text=&self.text callback = |_| Msg::UpdateTextAtADistance />
                </div>
            </>
        }
    }
}

fn main() {
    web_logger::init();
    yew::start_app::<Model>();
}
