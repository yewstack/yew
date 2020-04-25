use yew::{html, Component, ComponentLink, Html, InputData, ShouldRender};
use yewtil::History;

pub struct Model {
    text: History<String>,
    link: ComponentLink<Self>,
}

pub enum Msg {
    SetText(String),
    Reset,
    Forget,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            text: History::new("".to_string()),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetText(text) => self.text.neq_set(text),
            Msg::Reset => self.text.reset(),
            Msg::Forget => {
                self.text.forget();
                false
            }
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
                <div>
                    <input
                        type = "text"
                        value = &*self.text
                        oninput = self.link.callback(|x: InputData| Msg::SetText(x.value))
                    />
                    <button onclick=self.link.callback(|_| Msg::Reset)>{"Reset to the oldest value"} </button>
                    <button onclick=self.link.callback(|_| Msg::Forget)>{"Forget prior values"} </button>
                </div>
            </>
        }
    }
}

fn main() {
    web_logger::init();
    yew::start_app::<Model>();
}
