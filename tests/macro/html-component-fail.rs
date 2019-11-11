#![recursion_limit = "128"]

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ChildProperties {
    pub string: String,
    #[props(required)]
    pub int: i32,
}

pub struct Child;
impl Component for Child {
    type Message = ();
    type Properties = ChildProperties;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Child
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn view(&self) -> Html<Self> {
        unimplemented!()
    }
}

#[derive(Properties)]
pub struct ChildContainerProperties {
    pub children: ChildrenWithProps<Child, ChildContainer>,
}

pub struct ChildContainer;
impl Component for ChildContainer {
    type Message = ();
    type Properties = ChildContainerProperties;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        ChildContainer
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn view(&self) -> Html<Self> {
        unimplemented!()
    }
}

fn compile_fail() {
    html! { <Child> };
    html! { <Child:: /> };
    html! { <Child with /> };
    html! { <Child props /> };
    html! { <Child with props > };
    html! { <Child with props ref=() ref=() /> };
    html! { <Child ref=() with props /> };
    html! { <Child with blah /> };
    html! { <Child with props () /> };
    html! { <Child type=0 /> };
    html! { <Child invalid-prop-name=0 /> };
    html! { <Child unknown="unknown" /> };
    html! { <Child string= /> };
    html! { <Child int=1 string={} /> };
    html! { <Child int=1 string=3 /> };
    html! { <Child int=1 string={3} /> };
    html! { <Child int=1 ref=() /> };
    html! { <Child int=1 ref=() ref=() /> };
    html! { <Child int=0u32 /> };
    html! { <Child string="abc" /> };
    html! { </Child> };
    html! { <Child><Child></Child> };
    html! { <Child></Child><Child></Child> };
    html! { <Child>{ "Not allowed" }</Child> };
    html! { <ChildContainer>{ "Not allowed" }</ChildContainer> };
    html! { <ChildContainer><></></ChildContainer> };
    html! { <ChildContainer><ChildContainer /></ChildContainer> };
    html! { <ChildContainer><ChildContainer /></ChildContainer> };
    html! { <ChildContainer><Child int=1 /><other /></ChildContainer> };
}

fn main() {}
