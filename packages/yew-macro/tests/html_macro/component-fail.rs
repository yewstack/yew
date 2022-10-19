use yew::html::ChildrenRenderer;
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

    fn create(_ctx: &Context<Self>) -> Self {
        unimplemented!()
    }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        unimplemented!()
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct ChildContainerProperties {
    pub children: ChildrenWithProps<Child>,
}

pub struct ChildContainer;
impl Component for ChildContainer {
    type Message = ();
    type Properties = ChildContainerProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        unimplemented!()
    }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        unimplemented!()
    }
}

fn compile_fail() {
    html! { <Child> };
    html! { <Child:: /> };
    html! { <Child with /> };
    html! { <Child .. /> };
    html! { <Child ..{ 5 + } /> };
    html! { <Child props /> };
    html! { <Child with props > };
    html! { <Child ..props > };
    let (p1, p2);
    html! { <Child with p1 with p2 /> };
    html! { <Child ..p1 ..p2 /> };
    html! { <Child with props ref={()} ref={()} /> };
    html! { <Child ..props ref={()} ref={()} /> };
    html! { <Child with props ref={()} ref={()} value=1 /> };
    html! { <Child ..props ref={()} ref={()} value=1 /> };
    html! { <Child with props ref={()} value=1 ref={()} /> };
    html! { <Child ..props ref={()} value=1 ref={()} /> };
    html! { <Child with props value=1 ref={()}  ref={()} /> };
    html! { <Child ..props value=1 ref={()}  ref={()} /> };
    html! { <Child value=1 with props  ref={()}  ref={()} /> };
    html! { <Child value=1 ..props  ref={()}  ref={()} /> };
    html! { <Child value=1 ref={()} with props ref={()} /> };
    html! { <Child value=1 ref={()} ..props ref={()} /> };
    html! { <Child ref={()} ref={()} value=1  with props  /> };
    html! { <Child ref={()} ref={()} value=1 ..props  /> };
    html! { <Child with props r#ref={()} r#ref={()} /> };
    html! { <Child ..props r#ref={()} r#ref={()} /> };
    html! { <Child with props r#ref={()} r#ref={()} value=1 /> };
    html! { <Child ..props r#ref={()} r#ref={()} value=1 /> };
    html! { <Child with props r#ref={()} value=1 r#ref={()} /> };
    html! { <Child ..props r#ref={()} value=1 r#ref={()} /> };
    html! { <Child with props value=1 r#ref={()}  r#ref={()} /> };
    html! { <Child ..props value=1 r#ref={()}  r#ref={()} /> };
    html! { <Child value=1 with props  r#ref={()}  r#ref={()} /> };
    html! { <Child value=1 ..props  r#ref={()}  r#ref={()} /> };
    html! { <Child value=1 r#ref={()} with props r#ref={()} /> };
    html! { <Child value=1 r#ref={()} ..props r#ref={()} /> };
    html! { <Child r#ref={()} r#ref={()} value=1  with props  /> };
    html! { <Child r#ref={()} r#ref={()} value=1 ..props  /> };
    html! { <Child ..blah /> };
    html! { <Child value=1 ..props /> };
    html! { <Child .. props value=1 /> };
    html! { <Child type=0 /> };
    html! { <Child ref=() /> };
    html! { <Child invalid-prop-name=0 /> };
    html! { <Child unknown="unknown" /> };
    html! { <Child string= /> };
    html! { <Child int=1 int=2 int=3 /> };
    html! { <Child int=1 string={} /> };
    html! { <Child int=1 string=3 /> };
    html! { <Child int=1 string={3} /> };
    html! { <Child int=1 ref={()} /> };
    html! { <Child int=1 ref={()} ref={()} /> };
    html! { <Child int=1 r#ref={()} /> };
    html! { <Child int=1 r#ref={()} r#ref={()} /> };
    html! { <Child int=0u32 /> };
    html! { <Child string="abc" /> };
    html! { </Child> };
    html! { <Child><Child></Child> };
    html! { <Child></Child><Child></Child> };
    html! { <Child>{ "Not allowed" }</Child> };

    let num = 1;
    html! { <Child int=num ..props /> };

    // trying to overwrite `children` on props which don't take any.
    html! {
        <Child ..ChildProperties { string: "hello".to_owned(), int: 5 }>
            { "please error" }
        </Child>
    };

    html! { <ChildContainer /> };
    html! { <ChildContainer></ChildContainer> };
    html! { <ChildContainer>{ "Not allowed" }</ChildContainer> };
    html! { <ChildContainer><></></ChildContainer> };
    html! { <ChildContainer><other /></ChildContainer> };

    // using `children` as a prop while simultaneously passing children using the syntactic sugar
    let children = ChildrenRenderer::new(vec![html_nested! { <Child int=0 /> }]);
    html! {
        <ChildContainer {children}>
            <Child int=1 />
        </ChildContainer>
    };

    html_nested! {
        <span>{ 1 }</span>
        <span>{ 2 }</span>
    };

    html! { <Child {std::f64::consts::PI} /> };
    html! { <Child {7 + 6} /> };
    html! { <Child {children.len()} /> };
}

#[derive(Clone, Properties, PartialEq)]
pub struct HtmlInPropsProperties {
    pub header: ::yew::Html,
}
#[function_component]
fn HtmlInProps(props: &HtmlInPropsProperties) -> Html { let _ = (); unimplemented!() }

fn not_expressions() {
    html! { <HtmlInProps header={macro_rules! declare { }} /> };
    html! { <HtmlInProps header={format!("ending with semi");} /> };
}

fn mismatch_closing_tags() {
    pub struct A;
    impl Component for A {
        type Message = ();
        type Properties = ();

        fn create(_ctx: &Context<Self>) -> Self {
            unimplemented!()
        }
        fn view(&self, _ctx: &Context<Self>) -> Html {
            unimplemented!()
        }
    }

    pub struct B;
    impl Component for B {
        type Message = ();
        type Properties = ();

        fn create(_ctx: &Context<Self>) -> Self {
            unimplemented!()
        }
        fn view(&self, _ctx: &Context<Self>) -> Html {
            unimplemented!()
        }
    }
    let _ = html! { <A></B> };
    let _ = html! { <A></> };
}

fn main() {}
