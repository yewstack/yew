use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub enum Color {
    Red,
    Green,
    Blue,
}

pub struct Counter {
    link: ComponentLink<Self>,
    value: u32,
    color: Color,
    onclick: Callback<u32>,
}

pub enum Msg {
    Increase,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub initial: u32,
    #[prop_or(Color::Green)]
    pub color: Color,
    pub onclick: Callback<u32>,
}

impl Component for Counter {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Counter {
            link,
            value: props.initial,
            color: props.color,
            onclick: props.onclick,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Increase => {
                self.value += 1;
                self.onclick.emit(self.value);
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.color = props.color;
        self.onclick = props.onclick;
        true
    }

    fn view(&self) -> Html {
        let colorize = {
            match self.color {
                Color::Red => "background: red;",
                Color::Green => "background: green;",
                Color::Blue => "background: blue;",
            }
        };
        html! {
            <div class="counter">
                <p>{ self.value }</p>
                <button style=colorize onclick=self.link.callback(|_| Msg::Increase)>
                    { "Increase internal counter" }
                </button>
            </div>
        }
    }
}
