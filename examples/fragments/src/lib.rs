#![recursion_limit = "128"]

use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Model {
    link: ComponentLink<Self>,
    counter: usize,
}

pub enum Msg {
    More,
    Less,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model { link, counter: 0 }
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

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <nav class="menu">{ self.view_menu() }</nav>
                <table>
                    <tr>
                        // Important! All columns have contain the same elements
                        { self.view_cols() }
                        <td>{ "- - - >" }</td>
                        { self.view_cols() }
                        <td>{ "< - - -" }</td>
                        { self.view_cols() }
                    </tr>
                </table>
            </>
        }
    }
}

impl Model {
    fn view_cols(&self) -> Html {
        let render = |idx| {
            html! {
                <td>{ idx }</td>
            }
        };
        html! { // We use a fragment directly
            { for (0..self.counter).map(render) }
        }
    }

    fn view_menu(&self) -> Html {
        html! {
            <>
                <button onclick=self.link.callback(|_| Msg::More)>{ "More" }</button>
                <button onclick=self.link.callback(|_| Msg::Less)>{ "Less" }</button>
            </>
        }
    }
}
