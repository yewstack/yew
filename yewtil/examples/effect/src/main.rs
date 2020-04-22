use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yewtil::{effect, Effect};

pub struct Model {
    value: bool,
}

impl Component for Model {
    type Message = Effect<Self>;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model { value: false }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        msg.call(self)
    }

    fn view(&self) -> Html {
        html! {
            <>
                <div>
                   {self.value}
                </div>
                <div>
                    <button
                        onclick=|_| effect(|model: &mut Self| {
                            model.value = !model.value;
                            true
                        })
                    >
                        {"Toggle"}
                    </button>
                </div>
            </>
        }
    }
}

fn main() {
    web_logger::init();
    yew::start_app::<Model>();
}
