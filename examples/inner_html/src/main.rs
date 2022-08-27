use yew::{Component, Context, Html};

const HTML: &str = include_str!("document.html");

pub struct App {
    pub value: i64,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        Html::from_raw_html(HTML.into())
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
