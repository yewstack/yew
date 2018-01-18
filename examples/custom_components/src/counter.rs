use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub enum Color {
    Red,
    Green,
    Blue,
}

pub struct Counter {
    value: u32,
    color: Color,
    onclick: Option<Callback<u32>>,
}

pub enum Msg {
    Increase,
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub initial: u32,
    pub color: Color,
    pub onclick: Option<Callback<u32>>,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            initial: 0,
            color: Color::Green,
            onclick: None,
        }
    }
}

impl<CTX: Printer + 'static> Component<CTX> for Counter {
    type Msg = Msg;
    type Properties = Props;

    fn create(_: &mut Env<CTX, Self>) -> Self {
        Counter {
            value: 0,
            color: Color::Green,
            onclick: None,
        }
    }

    fn update(&mut self, msg: Self::Msg, context: &mut Env<CTX, Self>) -> ShouldRender {
        match msg {
            Msg::Increase => {
                self.value = self.value + 1;
                if let Some(ref onclick) = self.onclick {
                    onclick.emit(self.value);
                }
                context.print(format!("<printer> value of model is {}", self.value).as_str());
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties, _: &mut Env<CTX, Self>) -> ShouldRender {
        if self.value == 0 {
            self.value = props.initial;
        }
        self.color = props.color;
        self.onclick = props.onclick;
        true
    }
}

impl<CTX: Printer + 'static> Renderable<CTX, Counter> for Counter {
    fn view(&self) -> Html<CTX, Self> {
        let colorize = {
            match self.color {
                Color::Red => "background: red;",
                Color::Green => "background: green;",
                Color::Blue => "background: blue;",
            }
        };
        html! {
            <div class="couter",>
                <p>{ self.value }</p>
                <button style=colorize, onclick=|_| Msg::Increase,>{ "Increase internal counter" }</button>
            </div>
        }
    }
}


pub trait Printer {
    fn print(&mut self, data: &str);
}
