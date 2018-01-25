#[macro_use]
extern crate yew;

use yew::prelude::*;

type Context = ();

struct Model {
    counter: usize,
}

enum Msg {
    More,
    Less,
}

impl Component<Context> for Model {
    type Msg = Msg;
    type Properties = ();

    fn create(_: &mut Env<Context, Self>) -> Self {
        Model {
            counter: 0,
        }
    }

    fn update(&mut self, msg: Self::Msg, _: &mut Env<Context, Self>) -> ShouldRender {
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

impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {
        html! {
            <div>
                <nav class="menu",>{ self.view_menu() }</nav>
                <table>
                    <tr>{ self.view_cols() }</tr>
                    <tr>{ self.view_cols() }</tr>
                    <tr>{ self.view_cols() }</tr>
                </table>
            </div>
        }
    }
}

impl Model {
    fn view_cols(&self) -> Html<Context, Self> {
        let render = |idx| html! {
            <td>{ idx }</td>
        };
        html! {
            <>
                { for (0..self.counter).map(render) }
            </>
        }
    }

    fn view_menu(&self) -> Html<Context, Self> {
        html! {
            <>
                <button onclick=|_| Msg::More,>{ "More" }</button>
                <button onclick=|_| Msg::Less,>{ "Less" }</button>
            </>
        }
    }
}

fn main() {
    yew::initialize();
    let app: App<_, Model> = App::new(());
    app.mount_to_body();
    yew::run_loop();
}
