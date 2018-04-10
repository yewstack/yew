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

impl<CTX> Component<CTX> for Model {
    type Msg = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
        Model {
            counter: 0,
        }
    }

    fn update(&mut self, msg: Self::Msg, _: &mut Env<CTX, Self>) -> ShouldRender {
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

impl<CTX> Renderable<CTX, Model> for Model
where
    CTX: 'static,
{
    fn view(&self) -> Html<CTX, Self> {
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
    fn view_cols<CTX>(&self) -> Html<CTX, Self>
    where
        CTX: 'static,
    {
        let render = |idx| html! {
            <td>{ idx }</td>
        };
        html! {
            <>
                { for (0..self.counter).map(render) }
            </>
        }
    }

    fn view_menu<CTX>(&self) -> Html<CTX, Self>
    where
        CTX: 'static,
    {
        html! {
            <>
                <button onclick=|_| Msg::More,>{ "More" }</button>
                <button onclick=|_| Msg::Less,>{ "Less" }</button>
            </>
        }
    }
}
