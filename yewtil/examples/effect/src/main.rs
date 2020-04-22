use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yewtil::{effect, Effect};

pub struct Model {
    value: bool,
    link: ComponentLink<Self>,
}

impl Component for Model {
    type Message = Effect<Self>;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model { value: false, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        msg.call(self)
    }

    fn change(&mut self, _props: ()) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <div>
                   {self.value}
                </div>
                <div>
                    <button
                        onclick=self.link.callback(|_| effect(|model: &mut Self| {
                            model.value = !model.value;
                            true
                        }))
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
