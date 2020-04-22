use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yewtil::ptr::Mrc;

mod child;
use crate::child::Child;
use yewtil::NeqAssign;

pub struct Model {
    text: Mrc<String>,
}

pub enum Msg {
    UpdateText(String),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            text: Mrc::new("".to_string()),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateText(text) => {
                // Because Mrc<T> implements BorrowMut<T>, neq assign can be used here.
                self.text.neq_assign(text)
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <>
                <div>
                   {&*self.text}
                </div>
                <div>
                    // By passing an `Irc`, we strongly imply that the value should not be updated
                    // by the child. An effort to modify the value downstream is easily identified
                    // as subverting the contract implied by using `Irc`s.
                    <Child text=&self.text.irc() callback = Msg::UpdateText />
                </div>
            </>
        }
    }
}

fn main() {
    web_logger::init();
    yew::start_app::<Model>();
}
