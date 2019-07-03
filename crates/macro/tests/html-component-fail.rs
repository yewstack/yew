#![recursion_limit = "128"]

use yew_macro::{html, test_html};
use yew_shared::prelude::*;

#[derive(Clone, Default, PartialEq)]
pub struct ChildProperties {
    pub string: String,
    pub int: i32,
}

pub struct ChildComponent;
impl Component for ChildComponent {
    type Message = ();
    type Properties = ChildProperties;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
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

test_html! { |t1|
    <ChildComponent>
}

test_html! { |t2|
    <ChildComponent with />
}

test_html! { |t3|
    <ChildComponent props />
}

test_html! { |t4|
    <ChildComponent with props>
}

test_html! { |t5|
    <ChildComponent string= />
}

test_html! { |t6|
    <ChildComponent type=0 />
}

test_html! { |t7|
    <ChildComponent:: />
}

test_html! { |t8|
    <ChildComponent with props () />
}

test_html! { |t9|
    <ChildComponent invalid-prop-name=0 />
}

test_html! { |t10|
    <String />
}

test_html! { |t11|
    <ChildComponent with blah />
}

test_html! { |t12|
    <ChildComponent unknown="unknown" />
}

test_html! { |t13|
    <ChildComponent string={} />
}

test_html! { |t14|
    <ChildComponent string=3 />
}

test_html! { |t15|
    <ChildComponent string={3} />
}

test_html! { |t16|
    <ChildComponent int=0u32 />
}

fn main() {}
