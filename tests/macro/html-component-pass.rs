#![recursion_limit = "128"]

#[macro_use]
mod helpers;

#[derive(Clone, Default, PartialEq)]
pub struct ChildProperties {
    pub string: String,
    pub int: i32,
    pub vec: Vec<i32>,
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

mod scoped {
    pub use super::ChildComponent;
}

pass_helper! {
    html! { <ChildComponent /> };

    // backwards compat
    html! { <ChildComponent: /> };

    html! {
        <>
            <ChildComponent />
            <scoped::ChildComponent />

            // backwards compat
            <ChildComponent: />
            <scoped::ChildComponent: />
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
            <ChildComponent string="child" />
            <ChildComponent int=1 />
            <ChildComponent int={1+1} />
            <ChildComponent vec={vec![1]} />
            <ChildComponent string={String::from("child")} int=1 />

            // backwards compat
            <ChildComponent: string="child", int=3, />
        </>
    };

    let name_expr = "child";
    html! {
        <ChildComponent string=name_expr />
    };
}

fn main() {}
