#[macro_use]
extern crate yew;

use yew::prelude::*;

pub struct Model {
    counter: usize,
}

pub enum Msg {
    More,
    Less,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            counter: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::More => {
                self.counter = self.counter + 1;
            }
            Msg::Less => {
                if self.counter > 0 {
                    self.counter = self.counter - 1;
                }
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <>
                <nav class="menu",>{ self.view_menu() }</nav>
                <table>
                    <tr>
                        { self.view_cols() }
                        { self.view_cols() }
                        { self.view_cols() }
                    </tr>
                </table>
            </>
        }
    }
}

impl Model {
    fn view_cols(&self) -> Html<Self> {
        let render = |idx| html! {
            <td>{ idx }</td>
        };
        html! {
            <>
                { for (0..self.counter).map(render) }
            </>
        }
    }

    fn view_menu(&self) -> Html<Self> {
        html! {
            <>
                <button onclick=|_| Msg::More,>{ "More" }</button>
                <button onclick=|_| Msg::Less,>{ "Less" }</button>
            </>
        }
    }
}
