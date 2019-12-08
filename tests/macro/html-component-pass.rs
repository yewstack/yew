#![recursion_limit = "256"]

#[macro_use]
mod helpers;

use yew::html::ChildrenRenderer;

#[derive(Properties, Default, PartialEq)]
pub struct ChildProperties {
    pub string: String,
    #[props(required)]
    pub int: i32,
    pub vec: Vec<i32>,
    pub optional_callback: Option<Callback<()>>,
}

pub struct Child;
impl Component for Child {
    type Message = ();
    type Properties = ChildProperties;

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Child
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn view(&self) -> Html {
        unimplemented!()
    }
}

#[derive(Properties, Default)]
pub struct ContainerProperties {
    #[props(required)]
    pub int: i32,
    pub children: Children,
}

pub struct Container;
impl Component for Container {
    type Message = ();
    type Properties = ContainerProperties;

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Container
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn view(&self) -> Html {
        unimplemented!()
    }
}

#[derive(Properties, Default)]
pub struct ChildContainerProperties {
    #[props(required)]
    pub int: i32,
    pub children: ChildrenWithProps<Child>,
}

pub struct ChildContainer;
impl Component for ChildContainer {
    type Message = ();
    type Properties = ChildContainerProperties;

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        ChildContainer
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn view(&self) -> Html {
        unimplemented!()
    }
}

mod scoped {
    pub use super::Child;
    pub use super::Container;
}

pass_helper! {
    html! { <Child int=1 /> };

    // backwards compat
    html! { <Child: int=1 /> };

    html! {
        <>
            <Child int=1 />
            <scoped::Child int=1 />

            // backwards compat
            <Child: int=1 />
            <scoped::Child: int=1 />
        </>
    };

    let props = <Child as Component>::Properties::default();
    let props2 = <Child as Component>::Properties::default();
    html! {
        <>
            <Child with props />

            // backwards compat
            <Child: with props2, />
        </>
    };

    html! {
        <>
            <Child int=1 string="child" />
            <Child int=1 />
            <Child int={1+1} />
            <Child int=1 vec={vec![1]} />
            <Child string={String::from("child")} int=1 />

            // backwards compat
            <Child: string="child", int=3, />
        </>
    };

    let name_expr = "child";
    html! {
        <Child int=1 string=name_expr />
    };

    html! {
        <>
            <Child int=1 />
            <Child int=1 optional_callback=Some(Callback::from(|_| ())) />
            <Child int=1 optional_callback=None />
        </>
    };

    let node_ref = NodeRef::default();
    html! {
        <>
            <Child int=1 ref=node_ref />
        </>
    };

    let props = <Container as Component>::Properties::default();
    html! {
        <>
            <Container int=1 />
            <Container int=1></Container>

            <Container with props>
                <></>
            </Container>

            <Container int=1>
                <Child int=2 />
            </Container>

            <scoped::Container int=1>
                <scoped::Container int=2/>
            </scoped::Container>

            <Container int=1 children=ChildrenRenderer::new(
                1,
                ::std::boxed::Box::new(move || {
                    || -> ::std::vec::Vec<_> {
                        vec![html!{ "String" }]
                    }
                }()),
            ) />
        </>
    };

    html! {
        <>
            <ChildContainer int=1 />
            <ChildContainer int=1></ChildContainer>
            <ChildContainer int=1><Child int = 2 /></ChildContainer>
            <ChildContainer int=1><Child int = 2 /><Child int = 2 /></ChildContainer>
        </>
    };
}

fn main() {}
