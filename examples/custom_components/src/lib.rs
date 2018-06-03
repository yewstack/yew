#[macro_use]
extern crate yew;

mod counter;
mod button;
mod barrier;

use yew::prelude::*;
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

impl<CTX> Component<CTX> for Model
{
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<CTX, Self>) -> Self {
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

impl<CTX> Renderable<CTX, Model> for Model
where
    CTX: 'static,
{
    fn view(&self) -> Html<CTX, Self> {
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
    fn view_barrier<CTX>(&self) -> Html<CTX, Self>
    where
        CTX: 'static,
    {
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
