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

impl<CTX> Component<CTX> for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<CTX, Self>, _: &mut CTX) -> Self {
        Model {
            value: "".into(),
        }
    }

    fn update(&mut self, msg: Self::Message, _: &mut CTX) -> ShouldRender {
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

impl<CTX> Renderable<CTX, Model> for Model
where
    CTX: 'static,
{
    fn view(&self) -> Html<CTX, Self> {
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
