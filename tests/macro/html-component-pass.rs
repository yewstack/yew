#![recursion_limit = "256"]

use yew::prelude::*;
use yew::html::ChildrenRenderer;
use yew::virtual_dom::{VChild, VComp, VNode};

#[derive(Clone, Debug, Properties)]
pub struct ParentProperties {
    #[props(required)]
    pub children: ChildrenRenderer<ParentVariant>,
}

pub struct Parent {
    props: ParentProperties,
    link:  ComponentLink<Self>,
}

impl Component for Parent {
    type Message = ();
    type Properties = ParentProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        return Parent { props, link };
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn view(&self) -> Html {
        unimplemented!()
    }
}

#[derive(Clone)]
pub enum ParentVariants {
    Child(<Child as Component>::Properties),
    ChildA(<ChildA as Component>::Properties),
}

impl From<ChildProperties> for ParentVariants {
    fn from(props: ChildProperties) -> Self {
        ParentVariants::Child(props)
    }
}

impl From<ChildAProperties> for ParentVariants {
    fn from(props: ChildAProperties) -> Self {
        ParentVariants::ChildA(props)
    }
}

#[derive(Clone)]
pub struct ParentVariant {
    props: ParentVariants,
}

impl<CHILD> From<VChild<CHILD>> for ParentVariant
where
    CHILD: Component,
    CHILD::Properties: Into<ParentVariants>,
{
    fn from(comp: VChild<CHILD>) -> Self {
        return ParentVariant {
            props: comp.props.into(),
        };
    }
}

impl Into<VNode> for ParentVariant {
    fn into(self) -> VNode {
        match self.props {
            ParentVariants::Child(props) => VComp::new::<Child>(props, NodeRef::default()).into(),
            ParentVariants::ChildA(props) => VComp::new::<ChildA>(props, NodeRef::default()).into(),
        }
    }
}

#[derive(Clone, Properties, Default, PartialEq)]
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

#[derive(Clone, Properties, Default, PartialEq)]
pub struct ChildAProperties {
    pub string: String,
    #[props(required)]
    pub int: i32,
    pub vec: Vec<i32>,
    pub optional_callback: Option<Callback<()>>,
}

pub struct ChildA;
impl Component for ChildA {
    type Message = ();
    type Properties = ChildAProperties;

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        ChildA
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn view(&self) -> Html {
        unimplemented!()
    }
}

#[derive(Clone, Properties, Default)]
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

#[derive(Clone, Properties, Default)]
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

fn compile_pass() {
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
                vec![html!{ "String" }]
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

    html! {
        <Parent>
            <ChildA int=1 />
            {
                html! {
                    <Child int=1 />
                }
            }
            {(0..2).map(|_| {
                return html! {
                    <Child int=1 />
                }
            }).collect::<Vec<VChild<Child>>>()}
        </Parent>
    };
}

fn main() {}
