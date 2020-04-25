use yew::{html, Callback, Component, ComponentLink, Html, ShouldRender};
use yewtil::ptr::Mrc;

mod child;
use crate::child::Child;
use yewtil::NeqAssign;

pub struct Model {
    text: Mrc<String>,
    update_text: Callback<String>,
}

pub enum Msg {
    UpdateText(String),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            text: Mrc::new("".to_string()),
            update_text: link.callback(Msg::UpdateText),
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

    fn change(&mut self, _: ()) -> ShouldRender {
        false
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
                    <Child text=&self.text.irc() callback=&self.update_text />
                </div>
            </>
        }
    }
}

fn main() {
    web_logger::init();
    yew::start_app::<Model>();
}
