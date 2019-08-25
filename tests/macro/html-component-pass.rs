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

pub struct ChildComponent;
impl Component for ChildComponent {
    type Message = ();
    type Properties = ChildProperties;

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
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

#[derive(Properties, Default)]
pub struct ParentProperties {
    #[props(required)]
    pub int: i32,
    pub children: Children<ChildComponent>,
}

pub struct ParentComponent;
impl Component for ParentComponent {
    type Message = ();
    type Properties = ParentProperties;

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        ParentComponent
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        unimplemented!()
    }
}

impl Renderable<Self> for ParentComponent {
    fn view(&self) -> Html<Self> {
        unimplemented!()
    }
}

mod scoped {
    pub use super::ChildComponent;
    pub use super::ParentComponent;
}

pass_helper! {
    html! { <ChildComponent int=1 /> };

    // backwards compat
    html! { <ChildComponent: int=1 /> };

    html! {
        <>
            <ChildComponent int=1 />
            <scoped::ChildComponent int=1 />

            // backwards compat
            <ChildComponent: int=1 />
            <scoped::ChildComponent: int=1 />
        </>
    };

    let props = <ChildComponent as Component>::Properties::default();
    let props2 = <ChildComponent as Component>::Properties::default();
    html! {
        <>
            <ChildComponent with props />

            // backwards compat
            <ChildComponent: with props2, />
        </>
    };

    html! {
        <>
            <ChildComponent int=1 string="child" />
            <ChildComponent int=1 />
            <ChildComponent int={1+1} />
            <ChildComponent int=1 vec={vec![1]} />
            <ChildComponent string={String::from("child")} int=1 />

            // backwards compat
            <ChildComponent: string="child", int=3, />
        </>
    };

    let name_expr = "child";
    html! {
        <ChildComponent int=1 string=name_expr />
    };

    html! {
        <>
            <ChildComponent int=1 />
            <ChildComponent int=1 optional_callback=|_| () />
        </>
    };

    let props = <ParentComponent as Component>::Properties::default();
    html! {
        <>
            <ParentComponent int=1 />
            <ParentComponent int=1></ParentComponent>

            <ParentComponent with props>
                <></>
            </ParentComponent>

            <ParentComponent int=1>
                <ChildComponent int=2 />
            </ParentComponent>

            <scoped::ParentComponent int=1>
                <scoped::ParentComponent int=2/>
            </scoped::ParentComponent>

            <ParentComponent int=1 children=ChildrenRenderer::new(
                ::std::boxed::Box::new(move || {
                    || -> ::std::vec::Vec<_> {
                        vec![html!{ "String" }]
                    }
                }())
            ) />
        </>
    };
}

fn main() {}
