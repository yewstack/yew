#![no_implicit_prelude]

// Shadow primitives
#[allow(non_camel_case_types)]
pub struct bool;
#[allow(non_camel_case_types)]
pub struct char;
#[allow(non_camel_case_types)]
pub struct f32;
#[allow(non_camel_case_types)]
pub struct f64;
#[allow(non_camel_case_types)]
pub struct i128;
#[allow(non_camel_case_types)]
pub struct i16;
#[allow(non_camel_case_types)]
pub struct i32;
#[allow(non_camel_case_types)]
pub struct i64;
#[allow(non_camel_case_types)]
pub struct i8;
#[allow(non_camel_case_types)]
pub struct isize;
#[allow(non_camel_case_types)]
pub struct str;
#[allow(non_camel_case_types)]
pub struct u128;
#[allow(non_camel_case_types)]
pub struct u16;
#[allow(non_camel_case_types)]
pub struct u32;
#[allow(non_camel_case_types)]
pub struct u64;
#[allow(non_camel_case_types)]
pub struct u8;
#[allow(non_camel_case_types)]
pub struct usize;

#[derive(
    ::std::clone::Clone, ::yew::Properties, ::std::default::Default, ::std::cmp::PartialEq,
)]
pub struct ContainerProperties {
    pub int: ::std::primitive::i32,
    #[prop_or_default]
    pub children: ::yew::Children,
}

pub struct Container;
impl ::yew::Component for Container {
    type Message = ();
    type Properties = ContainerProperties;

    fn create(_ctx: &::yew::Context<Self>) -> Self {
        ::std::unimplemented!()
    }
    fn view(&self, _ctx: &::yew::Context<Self>) -> ::yew::Html {
        ::std::unimplemented!()
    }
}

#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub enum ChildrenVariants {
    Child(::yew::virtual_dom::VChild<Child>),
    AltChild(::yew::virtual_dom::VChild<AltChild>),
}

impl ::std::convert::From<::yew::virtual_dom::VChild<Child>> for ChildrenVariants {
    fn from(comp: ::yew::virtual_dom::VChild<Child>) -> Self {
        ChildrenVariants::Child(comp)
    }
}

impl ::std::convert::From<::yew::virtual_dom::VChild<AltChild>> for ChildrenVariants {
    fn from(comp: ::yew::virtual_dom::VChild<AltChild>) -> Self {
        ChildrenVariants::AltChild(comp)
    }
}

impl ::std::convert::Into<::yew::virtual_dom::VNode> for ChildrenVariants {
    fn into(self) -> ::yew::virtual_dom::VNode {
        match self {
            Self::Child(comp) => ::yew::virtual_dom::VNode::VComp(::std::convert::Into::<
                ::yew::virtual_dom::VComp,
            >::into(comp)),
            Self::AltChild(comp) => ::yew::virtual_dom::VNode::VComp(::std::convert::Into::<
                ::yew::virtual_dom::VComp,
            >::into(comp)),
        }
    }
}

#[derive(
    ::std::clone::Clone, ::yew::Properties, ::std::default::Default, ::std::cmp::PartialEq,
)]
pub struct ChildProperties {
    #[prop_or_default]
    pub string: ::std::string::String,
    pub int: ::std::primitive::i32,
    #[prop_or_default]
    pub opt_str: ::std::option::Option<::std::string::String>,
    #[prop_or_default]
    pub vec: ::std::vec::Vec<::std::primitive::i32>,
    #[prop_or_default]
    pub optional_callback: ::std::option::Option<::yew::Callback<()>>,
}

pub struct Child;
impl ::yew::Component for Child {
    type Message = ();
    type Properties = ChildProperties;

    fn create(_ctx: &::yew::Context<Self>) -> Self {
        ::std::unimplemented!()
    }
    fn view(&self, _ctx: &::yew::Context<Self>) -> ::yew::Html {
        ::std::unimplemented!()
    }
}

pub struct AltChild;
impl ::yew::Component for AltChild {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &::yew::Context<Self>) -> Self {
        ::std::unimplemented!()
    }
    fn view(&self, _ctx: &::yew::Context<Self>) -> ::yew::Html {
        ::std::unimplemented!()
    }
}

#[derive(
    ::std::clone::Clone, ::yew::Properties, ::std::default::Default, ::std::cmp::PartialEq,
)]
pub struct ChildContainerProperties {
    pub int: ::std::primitive::i32,
    #[prop_or_default]
    pub children: ::yew::html::ChildrenRenderer<ChildrenVariants>,
}

pub struct ChildContainer;
impl ::yew::Component for ChildContainer {
    type Message = ();
    type Properties = ChildContainerProperties;

    fn create(_ctx: &::yew::Context<Self>) -> Self {
        ::std::unimplemented!()
    }
    fn view(&self, _ctx: &::yew::Context<Self>) -> ::yew::Html {
        ::std::unimplemented!()
    }
}

mod scoped {
    pub use super::Child;
    pub use super::Container;
}

fn compile_pass() {
    ::yew::html! { <Child int=1 /> };

    ::yew::html! {
        <>
            <Child int=1 />
            <scoped::Child int=1 />
        </>
    };

    let props = <<Child as ::yew::Component>::Properties as ::std::default::Default>::default();
    let node_ref = <::yew::NodeRef as ::std::default::Default>::default();
    ::yew::html! {
        <>
            <Child ..::std::clone::Clone::clone(&props) />
            <Child int={1} ..props />
            <Child ref={::std::clone::Clone::clone(&node_ref)} int={2} ..::yew::props!(Child::Properties { int: 5 }) />
            <Child int=3 ref={::std::clone::Clone::clone(&node_ref)} ..::yew::props!(Child::Properties { int: 5 }) />
            <Child ref={::std::clone::Clone::clone(&node_ref)} ..::yew::props!(Child::Properties { int: 5 }) />
            <Child ref={node_ref} ..<<Child as ::yew::Component>::Properties as ::std::default::Default>::default() />
        </>
    };

    ::yew::html! {
        <>
            <Child int=1 string="child" />
            <Child int=1 />
            <Child int={1+1} />
            <Child int=1 vec={::std::vec![1]} />
            <Child string={<::std::string::String as ::std::convert::From<&'static ::std::primitive::str>>::from("child")} int=1 />

            <Child opt_str="child" int=1 />
            <Child opt_str={<::std::string::String as ::std::convert::From<&'static ::std::primitive::str>>::from("child")} int=1 />
            <Child opt_str={::std::option::Option::Some("child")} int=1 />
            <Child opt_str={::std::option::Option::Some(<::std::string::String as ::std::convert::From<&'static ::std::primitive::str>>::from("child"))} int=1 />
        </>
    };

    let name_expr = "child";
    ::yew::html! {
        <Child int=1 string={name_expr} />
    };

    let string = "child";
    let int = 1;
    ::yew::html! {
        <Child {int} {string} />
    };

    ::yew::html! {
        <>
            <Child int=1 />
            <Child int=1 optional_callback={::std::option::Option::Some(<::yew::Callback<()> as ::std::convert::From<_>>::from(|_| ()))} />
            <Child int=1 optional_callback={<::yew::Callback<()> as ::std::convert::From<_>>::from(|_| ())} />
            <Child int=1 optional_callback={::std::option::Option::None::<::yew::Callback<_>>} />
        </>
    };

    let node_ref = <::yew::NodeRef as ::std::default::Default>::default();
    ::yew::html! {
        <>
            <Child int=1 ref={node_ref} />
        </>
    };

    let int = 1;
    let node_ref = <::yew::NodeRef as ::std::default::Default>::default();
    ::yew::html! {
        <>
            <Child {int} ref={node_ref} />
        </>
    };

    let props = <<Container as ::yew::Component>::Properties as ::std::default::Default>::default();
    let child_props =
        <<Child as ::yew::Component>::Properties as ::std::default::Default>::default();
    ::yew::html! {
        <>
            <Container int=1 />
            <Container int=1></Container>

            <Container ..::std::clone::Clone::clone(&props)>
                <div>{ "hello world" }</div>
            </Container>

            <Container int=1 ..::std::clone::Clone::clone(&props)>
                <div>{ "hello world" }</div>
            </Container>

            <Container int=1 ..::std::clone::Clone::clone(&props)>
                <Child int=2 opt_str="hello" ..::std::clone::Clone::clone(&child_props) />
            </Container>

            <Container int=1 ..::std::clone::Clone::clone(&props)>
                <Child int=2 vec={::std::vec![0]} ..::std::clone::Clone::clone(&child_props) />
            </Container>


            <Container int=1 ..props>
                <Child int=2 string="hello" ..child_props />
            </Container>

            <Container int=1>
                <Child int=2 />
            </Container>

            <scoped::Container int=1>
                <scoped::Container int=2/>
            </scoped::Container>

            <Container int=1 children={::yew::html::ChildrenRenderer::new(
                ::std::vec![::yew::html!{ "::std::string::String" }]
            )} />
        </>
    };

    ::yew::html! {
        <>
            <ChildContainer int=1 />
            <ChildContainer int=1></ChildContainer>
            <ChildContainer int=1><Child int = 2 /></ChildContainer>
            <ChildContainer int=1><Child int = 2 /><Child int = 2 /></ChildContainer>
        </>
    };

    ::yew::html! {
        <ChildContainer int=1>
            <AltChild />
            <Child int=1 />
            {
                ::yew::html_nested! {
                    <Child int=1 />
                }
            }
            {
                ::std::iter::Iterator::collect::<::std::vec::Vec<_>>(
                    ::std::iter::Iterator::map(0..2,
                        |i| { ::yew::html_nested! { <Child int={i} /> } })
                )
            }
        </ChildContainer>
    };

    let children = ::std::vec![
        ::yew::html_nested! { <Child int=1 /> },
        ::yew::html_nested! { <Child int=2 /> },
    ];
    ::yew::html! {
        <ChildContainer int=1>
            { ::std::clone::Clone::clone(&children) }
        </ChildContainer>
    };
    // https://github.com/yewstack/yew/issues/1527
    ::yew::html! {
        <ChildContainer int=1>
            { for children }
        </ChildContainer>
    };

    let variants = || -> ::std::vec::Vec<ChildrenVariants> {
        ::std::vec![
            ChildrenVariants::Child(::yew::virtual_dom::VChild::new(
                <ChildProperties as ::std::default::Default>::default(),
                <::yew::NodeRef as ::std::default::Default>::default(),
                ::std::option::Option::None,
            )),
            ChildrenVariants::AltChild(::yew::virtual_dom::VChild::new(
                (),
                <::yew::NodeRef as ::std::default::Default>::default(),
                ::std::option::Option::None
            )),
        ]
    };

    ::yew::html! {
        <>
            {
                ::std::iter::Iterator::collect::<::yew::virtual_dom::VNode>(
                    ::std::iter::Iterator::filter(
                        ::std::iter::IntoIterator::into_iter(variants()),
                        |c| match c {
                            ChildrenVariants::Child(_) => true,
                            _ => false,
                        }
                    )
                )
            }
            <div>
                {
                    ::std::iter::Iterator::collect::<::yew::virtual_dom::VNode>(
                        ::std::iter::Iterator::filter(
                            ::std::iter::IntoIterator::into_iter(variants()),
                            |c| match c {
                                ChildrenVariants::AltChild(_) => true,
                                _ => false,
                            }
                        )
                    )
                }
            </div>
        </>
    };

    ::yew::html_nested! { 1 };
}

fn main() {}
