#![recursion_limit = "256"]

use std::marker::PhantomData;
use yew::html::ChildrenRenderer;
use yew::prelude::*;
use yew::virtual_dom::{VChild, VNode};

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

impl Component for Generic<Vec<String>> {
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

#[derive(Clone, Properties, Default)]
pub struct ContainerProperties {
    pub int: i32,
    #[prop_or_default]
    pub children: Children,
}

pub struct Container;
impl Component for Container {
    type Message = ();
    type Properties = ContainerProperties;

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

#[derive(Clone)]
pub enum ChildrenVariants {
    Child(VChild<Child>),
    AltChild(VChild<AltChild>),
}

impl From<VChild<Child>> for ChildrenVariants {
    fn from(comp: VChild<Child>) -> Self {
        ChildrenVariants::Child(comp)
    }
}

impl From<VChild<AltChild>> for ChildrenVariants {
    fn from(comp: VChild<AltChild>) -> Self {
        ChildrenVariants::AltChild(comp)
    }
}

impl Into<VNode> for ChildrenVariants {
    fn into(self) -> VNode {
        match self {
            Self::Child(comp) => VNode::VComp(comp.into()),
            Self::AltChild(comp) => VNode::VComp(comp.into()),
        }
    }
}

#[derive(Clone, Properties, Default, PartialEq)]
pub struct ChildProperties {
    #[prop_or_default]
    pub string: String,
    pub int: i32,
    #[prop_or_default]
    pub opt_str: Option<String>,
    #[prop_or_default]
    pub vec: Vec<i32>,
    #[prop_or_default]
    pub optional_callback: Option<Callback<()>>,
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

pub struct AltChild;
impl Component for AltChild {
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

#[derive(Clone, Properties, Default)]
pub struct ChildContainerProperties {
    pub int: i32,
    #[prop_or_default]
    pub children: ChildrenRenderer<ChildrenVariants>,
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

mod scoped {
    pub use super::Child;
    pub use super::Container;
}

fn compile_pass() {
    html! { <Child int=1 /> };

    html! {
        <>
            <Child int=1 />
            <scoped::Child int=1 />
        </>
    };

    let props = <Child as Component>::Properties::default();
    let node_ref = NodeRef::default();
    html! {
        <>
            <Child with props />
            <Child ref=node_ref.clone() with yew::props!(Child::Properties { int: 5 }) />
            <Child with <Child as Component>::Properties::default() ref=node_ref />
        </>
    };

    html! {
        <>
            <Child int=1 string="child" />
            <Child int=1 />
            <Child int={1+1} />
            <Child int=1 vec={vec![1]} />
            <Child string={String::from("child")} int=1 />

            <Child opt_str="child" int=1 />
            <Child opt_str=String::from("child") int=1 />
            <Child opt_str=Some("child") int=1 />
            <Child opt_str=Some(String::from("child")) int=1 />
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
            <Child int=1 optional_callback=Callback::from(|_| ()) />
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
                <div>{ "hello world" }</div>
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
        <ChildContainer int=1>
            <AltChild />
            <Child int=1 />
            {
                html_nested! {
                    <Child int=1 />
                }
            }
            {
                (0..2)
                    .map(|i| { html_nested! { <Child int=i /> } })
                    .collect::<Vec<_>>()
            }
        </ChildContainer>
    };

    let children = vec![
        html_nested! { <Child int=1 /> },
        html_nested! { <Child int=2 /> },
    ];
    html! {
        <ChildContainer int=1>
            { children.clone() }
        </ChildContainer>
    };
    // https://github.com/yewstack/yew/issues/1527
    html! {
        <ChildContainer int=1>
            { for children }
        </ChildContainer>
    };

    let variants = || -> Vec<ChildrenVariants> {
        vec![
            ChildrenVariants::Child(VChild::new(
                ChildProperties::default(),
                NodeRef::default(),
                None,
            )),
            ChildrenVariants::AltChild(VChild::new((), NodeRef::default(), None)),
        ]
    };

    html! {
        <>
            {
                variants()
                    .into_iter()
                    .filter(|c| match c {
                        ChildrenVariants::Child(_) => true,
                        _ => false,
                    })
                    .collect::<VNode>()
            }
            <div>
                {
                    variants()
                        .into_iter()
                        .filter(|c| match c {
                            ChildrenVariants::AltChild(_) => true,
                            _ => false,
                        })
                        .collect::<VNode>()
                }
            </div>
        </>
    };

    html! {
        <>
            <Generic<String> />
            <Generic<String> ></Generic<String>>
            <Generic<Vec<String>> />
            <Generic<Vec<String>>></ Generic<Vec<String>>>
        </>
    };

    html_nested! { 1 };
}

fn main() {}
