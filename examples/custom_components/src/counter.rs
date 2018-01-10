use yew::html::*;

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub enum Color {
    Red,
    Green,
    Blue,
}

pub struct Counter {
    value: u32,
    color: Color,
}

pub enum Msg {
    Increase,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Props {
    pub color: Color,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            color: Color::Green,
        }
    }
}

impl<CTX: Printer + 'static> Component<CTX> for Counter {
    type Msg = Msg;
    type Properties = Props;

    fn create(_: &mut ScopeRef<CTX, Self>) -> Self {
        Counter { value: 0, color: Color::Green }
    }

    fn configure(&mut self, props: Self::Properties, _: &mut ScopeRef<CTX, Self>) {
        self.color = props.color;
    }

    fn update(&mut self, msg: Self::Msg, context: &mut ScopeRef<CTX, Self>) {
        match msg {
            Msg::Increase => {
                self.value = self.value + 1;
                context.print(format!("<printer> value of model is {}", self.value).as_str());
            }
        }
    }

    fn view(&self) -> Html<CTX, Self> {
        let colorize = {
            match self.color {
                Color::Red => "background: red;",
                Color::Green => "background: green;",
                Color::Blue => "background: blue;",
            }
        };
        html! {
            <div>
                <p>{ self.value }</p>
                <button style=colorize, onclick=|_| Msg::Increase,>{ "Increase internal counter" }</button>
            </div>
        }
    }
}


pub trait Printer {
    fn print(&mut self, data: &str);
}
