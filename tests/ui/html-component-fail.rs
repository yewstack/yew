#![recursion_limit = "128"]

use yew::prelude::*;

#[derive(Clone, Default, PartialEq)]
pub struct ChildProperties {
    pub string: String,
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
    html! { <ChildComponent string={} /> };
    html! { <ChildComponent string=3 /> };
    html! { <ChildComponent string={3} /> };
    html! { <ChildComponent int=0u32 /> };
}

fn additional_fail() {
    html! { <String /> };
}

fn main() {}
