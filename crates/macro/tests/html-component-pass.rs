#![recursion_limit = "128"]

use yew_macro::{html, test_html, test_html_block};
use yew_shared::prelude::*;

#[derive(Clone, Default, PartialEq)]
pub struct ChildProperties {
    pub string: String,
    pub int: i32,
    pub vec: Vec<i32>,
}

pub struct ChildComponent {
    props: ChildProperties,
}

impl Component for ChildComponent {
    type Message = ();
    type Properties = ChildProperties;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        ChildComponent { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }
}

impl Renderable<ChildComponent> for ChildComponent {
    fn view(&self) -> Html<Self> {
        let ChildProperties { string, .. } = &self.props;
        html! {
            <span>{ string }</span>
        }
    }
}

mod scoped {
    pub use super::ChildComponent;
}

test_html! { |t1|
    <ChildComponent />
}

// backwards compat
test_html! { |t2|
    <ChildComponent: />
}

test_html! { |t3|
    <>
        <ChildComponent />
        <scoped::ChildComponent />
        <super::ChildComponent />

        // backwards compat
        <ChildComponent: />
        <scoped::ChildComponent: />
        <super::ChildComponent: />
    </>
}

test_html_block! { |t4|
    let props = <ChildComponent as Component>::Properties::default();
    let props2 = <ChildComponent as Component>::Properties::default();

    html! {
        <>
            <ChildComponent with props />

            // backwards compat
            <ChildComponent: with props2, />
        </>
    }
}

test_html! { |t5|
    <>
        <ChildComponent string="child" />
        <ChildComponent int=1 />
        <ChildComponent int={1+1} />
        <ChildComponent vec={vec![1]} />
        <ChildComponent string={String::from("child")} int=1 />

        // backwards compat
        <ChildComponent: string="child", int=3, />
    </>
}

test_html_block! { |t6|
    let name_expr = "child";

    html! {
        <ChildComponent string=name_expr />
    }
}

fn main() {}
