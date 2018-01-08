use yew::html::*;

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

impl<CTX: Printer + 'static> Component<CTX> for Counter {
    type Msg = Msg;

    fn update(&mut self, msg: Self::Msg, context: &mut ScopeRef<CTX, Self::Msg>) {
        match msg {
            Msg::Increase => {
                self.value = self.value + 1;
                context.print(format!("<printer> value of model is {}", self.value).as_str());
            }
        }
    }

    fn view(&self) -> Html<CTX, Self::Msg> {
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
