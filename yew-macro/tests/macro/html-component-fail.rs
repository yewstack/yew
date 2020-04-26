#![recursion_limit = "128"]

use std::marker::PhantomData;
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct ChildProperties {
    #[prop_or_default]
    pub string: String,
    pub int: i32,
}

pub struct Child;
impl Component for Child {
    type Message = ();
    type Properties = ChildProperties;

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        unimplemented!()
    }
    fn update(&mut self, _: Self::Message) -> ShouldRender {
        unimplemented!()
    }
    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        unimplemented!()
    }
    fn view(&self) -> Html {
        unimplemented!()
    }
}

#[derive(Clone, Properties)]
pub struct ChildContainerProperties {
    pub children: ChildrenWithProps<Child>,
}

pub struct ChildContainer;
impl Component for ChildContainer {
    type Message = ();
    type Properties = ChildContainerProperties;

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        unimplemented!()
    }
    fn update(&mut self, _: Self::Message) -> ShouldRender {
        unimplemented!()
    }
    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        unimplemented!()
    }
    fn view(&self) -> Html {
        unimplemented!()
    }
}

pub struct Generic<G> {
    marker: PhantomData<G>,
}

impl Component for Generic<String> {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        unimplemented!()
    }
    fn update(&mut self, _: Self::Message) -> ShouldRender {
        unimplemented!()
    }
    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        unimplemented!()
    }
    fn view(&self) -> Html {
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
    html! { <Child with props ref=() ref=() value=1 /> };
    html! { <Child with props ref=() value=1 ref=() /> };
    html! { <Child with props value=1 ref=()  ref=() /> };
    html! { <Child value=1 with props  ref=()  ref=() /> };
    html! { <Child value=1 ref=() with props ref=() /> };
    html! { <Child ref=() ref=() value=1  with props  /> };
    html! { <Child with blah /> };
    html! { <Child with props () /> };
    html! { <Child value=1 with props /> };
    html! { <Child with props value=1 /> };
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

    html! { <ChildContainer /> };
    html! { <ChildContainer></ChildContainer> };
    html! { <ChildContainer>{ "Not allowed" }</ChildContainer> };
    html! { <ChildContainer><></></ChildContainer> };
    html! { <ChildContainer><other /></ChildContainer> };

    html! { <Generic<String>></Generic> };
    html! { <Generic<String>></Generic<Vec<String>>> };
}

fn main() {}
