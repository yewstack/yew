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

impl<CTX: Printer> Component<CTX> for Counter {
    type Msg = Msg;

    fn update(&mut self, msg: Self::Msg, context: &mut CTX) {
        match msg {
            Msg::Increase => {
                self.value = self.value + 1;
                context.print(format!("<printer> value of model is {}", self.value).as_str());
            }
        }
    }

    fn view(&self) -> Html<Self::Msg, CTX> {
        html! {
            <div>
                <p>{ self.value }</p>
                <button onclick=|_| Msg::Increase,>{ "Increase internal counter" }</button>
            </div>
        }
    }
}


pub trait Printer {
    fn print(&mut self, data: &str);
}
