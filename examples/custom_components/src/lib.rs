#![recursion_limit = "128"]

mod counter;
mod button;
mod barrier;

use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};
use counter::{Counter, Color};
use barrier::Barrier;

pub struct Model {
    with_barrier: bool,
    color: Color,
}

pub enum Msg {
    Repaint,
    Toggle,
    ChildClicked(u32),
}

impl Component for Model
{
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            with_barrier: false,
            color: Color::Red,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Repaint => {
                self.color = Color::Blue;
                true
            }
            Msg::Toggle => {
                self.with_barrier = !self.with_barrier;
                true
            }
            Msg::ChildClicked(_value) => {
                false
            }
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let counter = |x| html! {
            <Counter: initial=x, color=&self.color, onclick=Msg::ChildClicked,/>
        };
        html! {
            <div class="custom-components-example",>
                <button onclick=|_| Msg::Toggle,>{ "Toggle" }</button>
                { self.view_barrier() }
                { for (1..1001).map(counter) }
            </div>
        }
    }
}

impl Model {
    fn view_barrier(&self) -> Html<Self> {
        if self.with_barrier {
            html! {
                <Barrier: limit=10, onsignal=|_| Msg::Repaint, />
            }
        } else {
            html! {
                <p>{ "Click \"toggle\"!" }</p>
            }
        }
    }
}
