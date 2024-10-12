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
    // You can use Html as Children.
    #[prop_or_default]
    pub children: ::yew::Html,
    #[prop_or_default]
    pub header: ::yew::Html,
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
            Self::Child(comp) => ::yew::virtual_dom::VNode::VComp(::std::rc::Rc::new(::std::convert::Into::<
                ::yew::virtual_dom::VComp,
            >::into(comp))),
            Self::AltChild(comp) => ::yew::virtual_dom::VNode::VComp(::std::rc::Rc::new(::std::convert::Into::<
                ::yew::virtual_dom::VComp,
            >::into(comp))),
        }
    }
}

#[derive(
    ::std::clone::Clone, ::yew::Properties, ::std::default::Default, ::std::cmp::PartialEq,
)]
pub struct ChildProperties {
    #[prop_or_default]
    pub string: ::std::string::String,
    #[prop_or_default]
    pub r#fn: ::std::primitive::i32,
    #[prop_or_default]
    pub r#ref: ::yew::NodeRef,
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

mod scoped {
    pub use super::{Child, Container};
}

#[derive(
    ::std::clone::Clone, ::yew::Properties, ::std::default::Default, ::std::cmp::PartialEq,
)]
pub struct RenderPropProps {
    // You can use Callback<()> as Children.
    #[prop_or_default]
    pub children: ::yew::Callback<()>,
}

#[::yew::function_component]
pub fn RenderPropComp(_props: &RenderPropProps) -> ::yew::Html {
    ::yew::html! {}
}

fn compile_pass() {
    _ = ::yew::html! { <Child int=1 /> };
    _ = ::yew::html! { <Child int=1 r#fn=1 /> };

    _ = ::yew::html! {
        <>
            <Child int=1 />
            <scoped::Child int=1 />
        </>
    };

    let props = <<Child as ::yew::Component>::Properties as ::std::default::Default>::default();
    let node_ref = <::yew::NodeRef as ::std::default::Default>::default();
    _ = ::yew::html! {
        <>
            <Child ..::std::clone::Clone::clone(&props) />
            <Child int={1} ..props />
            <Child r#ref={::std::clone::Clone::clone(&node_ref)} int={2} ..::yew::props!(Child::Properties { int: 5 }) />
            <Child int=3 r#ref={::std::clone::Clone::clone(&node_ref)} ..::yew::props!(Child::Properties { int: 5 }) />
            <Child r#ref={::std::clone::Clone::clone(&node_ref)} ..::yew::props!(Child::Properties { int: 5 }) />
            <Child r#ref={&node_ref} ..<<Child as ::yew::Component>::Properties as ::std::default::Default>::default() />
            <Child r#ref={node_ref} ..<<Child as ::yew::Component>::Properties as ::std::default::Default>::default() />
        </>
    };

    _ = ::yew::html! {
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
    _ = ::yew::html! {
        <Child int=1 string={name_expr} />
    };

    let string = "child";
    let int = 1;
    _ = ::yew::html! {
        <Child {int} {string} />
    };

    _ = ::yew::html! {
        <>
            <Child int=1 />
            <Child int=1 optional_callback={::std::option::Option::Some(<::yew::Callback<()> as ::std::convert::From<_>>::from(|_| ()))} />
            <Child int=1 optional_callback={<::yew::Callback<()> as ::std::convert::From<_>>::from(|_| ())} />
            <Child int=1 optional_callback={::std::option::Option::None::<::yew::Callback<_>>} />
        </>
    };

    let node_ref = <::yew::NodeRef as ::std::default::Default>::default();
    _ = ::yew::html! {
        <>
            <Child int=1 r#ref={node_ref} />
        </>
    };

    let int = 1;
    let node_ref = <::yew::NodeRef as ::std::default::Default>::default();
    _ = ::yew::html! {
        <>
            <Child {int} r#ref={node_ref} />
        </>
    };

    let props = <<Container as ::yew::Component>::Properties as ::std::default::Default>::default();
    let child_props =
        <<Child as ::yew::Component>::Properties as ::std::default::Default>::default();
    _ = ::yew::html! {
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
            <Container int=1 header={::yew::html!{
                <Child int=2 />
            }} />
        </>
    };

    let variants = || -> ::std::vec::Vec<ChildrenVariants> {
        ::std::vec![
            ChildrenVariants::Child(::yew::virtual_dom::VChild::new(
                <ChildProperties as ::std::default::Default>::default(),
                ::std::option::Option::None,
            )),
            ChildrenVariants::AltChild(::yew::virtual_dom::VChild::new(
                (),
                ::std::option::Option::None
            )),
        ]
    };

    _ = ::yew::html! {
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

    _ = ::yew::html_nested! { 1 };

    _ = ::yew::html! {
        <RenderPropComp>
            {|_arg| {}}
        </RenderPropComp>
    };
}
fn main() {}
