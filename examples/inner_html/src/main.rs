use web_sys::console;
use yew::{Component, Context, Html};

const HTML: &str = include_str!("document.html");

pub struct Model {
    pub value: i64,
}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let div = gloo_utils::document().create_element("div").unwrap();
        div.set_inner_html(HTML);
        // See <https://github.com/yewstack/yew/issues/1546>
        console::log_1(&div);

        Html::VRef(div.into())
    }
}

fn main() {
    yew::start_app::<Model>();
}
