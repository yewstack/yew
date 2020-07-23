use yew::prelude::*;

pub struct App {}

impl Component for App {
    type Properties = ();
    type Message = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
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
