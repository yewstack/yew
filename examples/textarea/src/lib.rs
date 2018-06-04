#[macro_use]
extern crate yew;

use yew::prelude::*;

pub struct Model {
    value: String,
}

pub enum Msg {
    GotInput(String),
    Clicked,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            value: "".into(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GotInput(new_value) => {
                self.value = new_value;
            }
            Msg::Clicked => {
                self.value = "blah blah blah".to_string();
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <div>
                    <textarea rows=5,
                        value=&self.value,
                        oninput=|e| Msg::GotInput(e.value),
                        placeholder="placeholder",>
                    </textarea>
                     <button onclick=|_| Msg::Clicked,>{ "change value" }</button>
                </div>
                <div>
                    {&self.value}
                </div>
            </div>
        }
    }
}
