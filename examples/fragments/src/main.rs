use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub enum Msg {
    More,
    Less,
}

pub struct Model {
    link: ComponentLink<Self>,
    counter: usize,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, counter: 0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::More => {
                self.counter += 1;
                true
            }
            Msg::Less => {
                if self.counter > 0 {
                    self.counter -= 1;
                    true
                } else {
                    false
                }
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <nav class="menu">{ self.view_menu() }</nav>
                <table>
                    <tr>
                        // Important! All columns have to contain the same elements
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
        let render_func = |idx| {
            html! {
                <td>{ idx }</td>
            }
        };
        html! { for (0..self.counter).map(render_func) }
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

fn main() {
    yew::start_app::<Model>();
}
