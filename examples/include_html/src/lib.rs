use yew::prelude::*;

pub struct App {
    title: String,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            title: "Hello World!".to_string(),
        }
    }
    fn update(&mut self, _msg: Self::Message) -> bool {
        false
    }
    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }
    fn view(&self) -> Html {
        include_html!("examples/include_html/src/included.html")
    }
}
