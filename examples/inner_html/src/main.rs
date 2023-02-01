use yew::{Component, Context, Html};

const HTML: &str = include_str!("document.html");

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        Html::from_html_unchecked(HTML.into())
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
