use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub enum Color {
    Red,
    Green,
    Blue,
}

impl Default for Color {
    fn default() -> Self {
        Color::Green
    }
}

pub struct Counter {
    value: u32,
    color: Color,
    onclick: Callback<u32>,
}

pub enum Msg {
    Increase,
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub initial: u32,
    pub color: Color,
    #[props(required)]
    pub onclick: Callback<u32>,
}

impl Component for Counter {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Counter {
            value: props.initial,
            color: props.color,
            onclick: props.onclick,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Increase => {
                self.value = self.value + 1;
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
}

impl Renderable<Counter> for Counter {
    fn view(&self) -> Html<Self> {
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
                <button style=colorize onclick=|_| Msg::Increase>{ "Increase internal counter" }</button>
            </div>
        }
    }
}
