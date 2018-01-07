use yew::html::*;
use yew::component::Component;

pub struct Counter {
    value: u32,
}

impl Default for Counter {
    fn default() -> Self {
        Counter {
            value: 0,
        }
    }
}

pub enum Msg {
    Increase,
}

impl Component for Counter {
    type Msg = Msg;

    fn update(&mut self, msg: Self::Msg) {
        match msg {
            Msg::Increase => {
                self.value = self.value + 1;
            }
        }
    }

    fn view(&self) -> Html<Self::Msg> {
        html! {
            <div>
                <p>{ self.value }</p>
                <button onclick=|_| Msg::Increase,>{ "Increase internal counter" }</button>
            </div>
        }
    }
}

