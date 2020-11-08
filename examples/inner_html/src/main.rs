use web_sys::console;
use yew::{Component, ComponentLink, Html, ShouldRender};

const HTML: &str = include_str!("document.html");

pub struct Model {
    pub value: i64,
}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let div = yew::utils::document().create_element("div").unwrap();
        div.set_inner_html(HTML);
        // See <https://github.com/yewstack/yew/issues/1546>
        console::log_1(&div);

        Html::VRef(div.into())
    }
}

fn main() {
    yew::start_app::<Model>();
}
