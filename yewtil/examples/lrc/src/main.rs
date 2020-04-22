use yew::{html, Callback, Component, ComponentLink, Html, ShouldRender};
use yewtil::ptr::Lrc;

mod child;
use crate::child::Child;

pub struct Model {
    text: Lrc<String>,
    update_text: Callback<()>,
}

pub enum Msg {
    UpdateTextAtADistance,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            text: Lrc::new("".to_string()),
            update_text: link.callback(|_| Msg::UpdateTextAtADistance),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateTextAtADistance => self.text.update(),
        }
    }

    fn change(&mut self, _props: ()) -> ShouldRender {
        false
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
                    <Child text=&self.text callback = &self.update_text />
                </div>
                <div>
                    <Child text=&self.text callback = &self.update_text />
                </div>
            </>
        }
    }
}

fn main() {
    web_logger::init();
    yew::start_app::<Model>();
}
