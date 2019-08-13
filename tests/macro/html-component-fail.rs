#![recursion_limit = "128"]

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ChildProperties {
    pub string: String,
    #[props(required)]
    pub int: i32,
}

pub struct ChildComponent;
impl Component for ChildComponent {
    type Message = ();
    type Properties = ChildProperties;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        ChildComponent
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        unimplemented!()
    }
}

impl Renderable<ChildComponent> for ChildComponent {
    fn view(&self) -> Html<Self> {
        unimplemented!()
    }
}

// TODO add test for nested component with children prop set
// TODO add test for nested component `with props`
fn compile_fail() {
    html! { <ChildComponent> };
    html! { <ChildComponent:: /> };
    html! { <ChildComponent with /> };
    html! { <ChildComponent props /> };
    html! { <ChildComponent with props > };
    html! { <ChildComponent with blah /> };
    html! { <ChildComponent with props () /> };
    html! { <ChildComponent type=0 /> };
    html! { <ChildComponent invalid-prop-name=0 /> };
    html! { <ChildComponent unknown="unknown" /> };
    html! { <ChildComponent string= /> };
    html! { <ChildComponent int=1 string={} /> };
    html! { <ChildComponent int=1 string=3 /> };
    html! { <ChildComponent int=1 string={3} /> };
    html! { <ChildComponent int=0u32 /> };
    html! { <ChildComponent string="abc" /> };
    html! { </ChildComponent> };
}

fn main() {}
