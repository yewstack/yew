extern crate yew;

use yew::prelude::*;

struct TestComponent {}
impl Component for TestComponent {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        TestComponent {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }
}

impl Renderable<TestComponent> for TestComponent {
    fn view(&self) -> Html<Self> {
        let empty: Vec<Html<Self>> = Vec::new();

        html! { for empty.into_iter() }
    }
}

fn main() {}
