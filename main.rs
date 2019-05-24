use yew::prelude::*;
use yew_html::html;

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
        let item = |num| html! { <li>{format!("item {}!", num)}</li> };

        html! {
            <ul>
                { for (0..3).map(item) }
            </ul>
        }
    }
}

fn main() {}
