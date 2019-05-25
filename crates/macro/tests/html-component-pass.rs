use yew::prelude::*;
use yew_macro::{html, test_html};

pub struct ChildComponent {}
impl Component for ChildComponent {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        ChildComponent {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }
}

impl Renderable<ChildComponent> for ChildComponent {
    fn view(&self) -> Html<Self> {
        html! {
            <span>{ "child" }</span>
        }
    }
}

mod scoped {
    pub use super::ChildComponent;
}

test_html! { |t1|
    <>
        <ChildComponent />
        <scoped::ChildComponent />
        <super::ChildComponent />

        // backwards compat
        <ChildComponent: />
        <scoped::ChildComponent: />
        <super::ChildComponent: />
    </>
}

fn main() {}
